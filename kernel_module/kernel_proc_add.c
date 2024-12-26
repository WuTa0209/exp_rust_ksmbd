#include <linux/init.h>
#include <linux/module.h>
#include <linux/proc_fs.h>
#include <linux/uaccess.h>
#include "checker.c"
#define PROC_NAME "foo"
#define BUFFER_SIZE 128

static char proc_buffer[BUFFER_SIZE];
static int result = 0;

static ssize_t proc_write(struct file *file, const char __user *buffer, size_t count, loff_t *pos) {
    char input[BUFFER_SIZE];
    int num1, num2;
    int arr[2] = {0};
    if (count >= BUFFER_SIZE)
        return -EINVAL;

    if (copy_from_user(input, buffer, count))
        return -EFAULT;

    input[count] = '\0';

    /* Parse two integers, there is BUG */
    if (sscanf(input, "%d %d", &num1, &num2) == 2) {
        printk(KERN_INFO "show index %d and index %d value, %d %d", num1, num2, arr[num1], arr[num2]);
        return count;
    }

    return -EINVAL;
}

static ssize_t proc_read(struct file *file, char __user *buffer, size_t count, loff_t *pos) {
    int len;

    if (*pos > 0 || count < BUFFER_SIZE)
        return 0;

    len = snprintf(proc_buffer, BUFFER_SIZE, "%d\n", result);

    if (copy_to_user(buffer, proc_buffer, len))
        return -EFAULT;

    *pos = len;
    return len;
}

static const struct proc_ops proc_fops = {
    .proc_read = proc_read,
    .proc_write = proc_write,
};

static int __init proc_init(void) {
    proc_create(PROC_NAME, 0666, NULL, &proc_fops);
    printk(KERN_INFO "/proc/%s created\n", PROC_NAME);
    return 0;
}

static void __exit proc_exit(void) {
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "/proc/%s removed\n", PROC_NAME);
}

module_init(proc_init);
module_exit(proc_exit);

MODULE_LICENSE("GPL");
