#include <linux/init.h>
#include <linux/module.h>
#include <linux/proc_fs.h>
#include <linux/uaccess.h>
#include <linux/bitops.h>
#include <linux/bug.h>
#include <linux/ctype.h>
#include <linux/kernel.h>
#include <linux/types.h>
#include <linux/sched.h>
#include <linux/ubsan.h>
#include <kunit/test-bug.h>
#include "ubsan.h"

#define VALUE_LENGTH 40
#define REPORTED_BIT 31
#define COLUMN_MASK (~(1U << REPORTED_BIT))
#define LINE_MASK   (~0U)
static bool was_reported(struct source_location *location)
{
	return test_and_set_bit(REPORTED_BIT, &location->reported);
}

static bool suppress_report(struct source_location *loc)
{
	return was_reported(loc);
}

static bool type_is_int(struct type_descriptor *type)
{
	return type->type_kind == type_kind_int;
}

static bool type_is_signed(struct type_descriptor *type)
{
	WARN_ON(!type_is_int(type));
	return  type->type_info & 1;
}

static unsigned type_bit_width(struct type_descriptor *type)
{
	return 1 << (type->type_info >> 1);
}

static bool is_inline_int(struct type_descriptor *type)
{
	unsigned inline_bits = sizeof(unsigned long)*8;
	unsigned bits = type_bit_width(type);

	WARN_ON(!type_is_int(type));

	return bits <= inline_bits;
}

static s_max get_signed_val(struct type_descriptor *type, void *val)
{
	if (is_inline_int(type)) {
		unsigned extra_bits = sizeof(s_max)*8 - type_bit_width(type);
		unsigned long ulong_val = (unsigned long)val;

		return ((s_max)ulong_val) << extra_bits >> extra_bits;
	}

	if (type_bit_width(type) == 64)
		return *(s64 *)val;

	return *(s_max *)val;
}

static bool val_is_negative(struct type_descriptor *type, void *val)
{
	return type_is_signed(type) && get_signed_val(type, val) < 0;
}

static u_max get_unsigned_val(struct type_descriptor *type, void *val)
{
	if (is_inline_int(type))
		return (unsigned long)val;

	if (type_bit_width(type) == 64)
		return *(u64 *)val;

	return *(u_max *)val;
}

static void val_to_string(char *str, size_t size, struct type_descriptor *type,
			void *value)
{
	if (type_is_int(type)) {
		if (type_bit_width(type) == 128) {
#if defined(CONFIG_ARCH_SUPPORTS_INT128)
			u_max val = get_unsigned_val(type, value);

			scnprintf(str, size, "0x%08x%08x%08x%08x",
				(u32)(val >> 96),
				(u32)(val >> 64),
				(u32)(val >> 32),
				(u32)(val));
#else
			WARN_ON(1);
#endif
		} else if (type_is_signed(type)) {
			scnprintf(str, size, "%lld",
				(s64)get_signed_val(type, value));
		} else {
			scnprintf(str, size, "%llu",
				(u64)get_unsigned_val(type, value));
		}
	}
}

static void ubsan_prologue(struct source_location *loc, const char *reason)
{
	// current->in_ubsan++;

	pr_warn(CUT_HERE);

	pr_err("UBSAN: %s in %s:%d:%d\n", reason, loc->file_name,
		loc->line & LINE_MASK, loc->column & COLUMN_MASK);

	kunit_fail_current_test("%s in %s", reason, loc->file_name);
}

static void ubsan_epilogue(void)
{
	dump_stack();
	pr_warn("---[ end trace ]---\n");

	// current->in_ubsan--;

	check_panic_on_warn("UBSAN");
}

static void handle_overflow(struct overflow_data *data, void *lhs,
			void *rhs, char op)
{

	struct type_descriptor *type = data->type;
	char lhs_val_str[VALUE_LENGTH];
	char rhs_val_str[VALUE_LENGTH];

	if (suppress_report(&data->location))
		return;

	ubsan_prologue(&data->location, type_is_signed(type) ?
			"signed-integer-overflow" :
			"unsigned-integer-overflow");

	val_to_string(lhs_val_str, sizeof(lhs_val_str), type, lhs);
	val_to_string(rhs_val_str, sizeof(rhs_val_str), type, rhs);
	pr_err("%s %c %s cannot be represented in type %s\n",
		lhs_val_str,
		op,
		rhs_val_str,
		type->type_name);

	ubsan_epilogue();
}


static void  __wrap___ubsan_handle_add_overflow(void *_data, void *lhs, void *rhs) {
    printk(KERN_INFO "\nOverflow detected!\n");
    dump_stack();
    while(1);
}

static void  __ubsan_handle_add_overflow(void *_data, void *lhs, void *rhs) {
    printk(KERN_INFO "\nOverflow detected!\n");
    dump_stack();
    while(1);
}

static void __ubsan_handle_divrem_overflow(void *_data, void *lhs, void *rhs) {
    struct overflow_data *data = _data;
	char rhs_val_str[VALUE_LENGTH];
    printk(KERN_INFO "\nDivide Overflow Detected\n");
    dump_stack();
    while(1);
}

static void __ubsan_handle_sub_overflow(void *_data, void *lhs, void *rhs) {
    printk(KERN_INFO "\nSubtract Overflow Detected\n");
    dump_stack();
    while(1);
}

static void __ubsan_handle_out_of_bounds(void *_data, void *lhs, void *rhs) {
    printk(KERN_INFO "\nArray Out of Bounds Detected\n");
    struct out_of_bounds_data *data = _data;
	char index_str[VALUE_LENGTH];

	if (suppress_report(&data->location))
		return;

	ubsan_prologue(&data->location, "array-index-out-of-bounds");

	val_to_string(index_str, sizeof(index_str), data->index_type, (void *)data);
	pr_err("index %s is out of range for type %s\n", index_str,
		data->array_type->type_name);
	ubsan_epilogue();
    dump_stack();
    printk(KERN_INFO "End of the hanlder\n");
}

static void __ubsan_handle_pointer_overflow(void *_data, void *lhs, void *rhs) {
    printk(KERN_INFO "\nPointer Overflow Detected\n");
    dump_stack();
    while(1);
}

static void __ubsan_handle_type_mismatch_v1(void *_data, void *lhs, void *rhs) {
    printk(KERN_INFO "\nType Mismatch Detected\n");
    dump_stack();
    while(1);
}

static void __ubsan_handle_load_invalid_value(void *_data, void *lhs, void *rhs) {
    printk(KERN_INFO "\nInvalid Value Detected\n");
    dump_stack();
    while(1);
}

static void __ubsan_handle_shift_out_of_bounds(void *_data, void *lhs, void *rhs) {
    printk(KERN_INFO "\nShift Out of Bounds Detected\n");
    dump_stack();
    while(1);
}

static void __ubsan_handle_builtin_unreachable(void *_data, void *lhs, void *rhs) {
    printk(KERN_INFO "\nUnreachable Code Detected\n");
    dump_stack();
    while(1);
}

void check_panic_on_warn(const char *reason)
{
    if (panic_on_warn) {
        pr_emerg("UBSAN: %s detected\n", reason);
        panic("UBSAN: %s detected\n", reason);
    }
}