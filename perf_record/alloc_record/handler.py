import re
import matplotlib.pyplot as plt


file_path = 'C_res.txt' 
file_path1 = 'Rust_res.txt'
file_path2 = 'Rust_without_overflow.txt'
total_time = 0
total_time1 = 0
total_time2 = 0
C_time = []
Rust_time = []
Rust_without_overflow = []
with open(file_path, 'r') as file:
    lines = file.readlines()
with open(file_path1, 'r') as file:
    lines1 = file.readlines()
with open(file_path2, 'r') as file:
    lines2 = file.readlines()

pattern = r"(\d+\.\d+) us"
for i in lines:
    matches = re.findall(pattern, i)
    for match in matches:
        total_time += float(match)
        C_time.append(float(match))

for i in lines1:
    matches = re.findall(pattern, i)
    for match in matches:
        total_time1 += float(match)
        Rust_time.append(float(match))

for i in lines2:
    matches = re.findall(pattern, i)
    for match in matches:
        total_time2 += float(match)
        Rust_without_overflow.append(float(match))

print("C Avg:", total_time/len(C_time))
# get standard deviation from C_time list
mean = sum(C_time) / len(C_time)
variance = sum([((x - mean) ** 2) for x in C_time]) / len(C_time)
res = variance ** 0.5
print("C Standard deviation:", res/len(C_time))


print("Rust Avg:", total_time1/len(Rust_time))
# get standard deviation from Rust_time list
mean = sum(Rust_time) / len(Rust_time)
variance = sum([((x - mean) ** 2) for x in Rust_time]) / len(Rust_time)
res = variance ** 0.5
print("Rust Standard deviation:", res/len(Rust_time))

print("Rust without overflow Avg:", total_time2/len(Rust_without_overflow))
# get standard deviation from Rust_without_overflow list
mean = sum(Rust_without_overflow) / len(Rust_without_overflow)
variance = sum([((x - mean) ** 2) for x in Rust_without_overflow]) / len(Rust_without_overflow)
res = variance ** 0.5
print("Rust without overflow Standard deviation:", res/len(Rust_without_overflow))


# x_coords1 = [i for i in range(100)]
# y_coords1 = [i for i in C_time]
# plt.plot(x_coords1, y_coords1,  linestyle='-', color='blue')

x_coords2 = [i for i in range(100)]
y_coords2 = [i for i in Rust_time]
plt.plot(x_coords2, y_coords2,  linestyle='-', color='orange')

x_coords3 = [i for i in range(100)]
y_coords3 = [i for i in Rust_without_overflow]
plt.plot(x_coords3, y_coords3,  linestyle='-', color='green')

# 添加標籤和標題
plt.xlabel('times')
plt.ylabel('time(us)')
plt.title('C vs Rust in alloc')



# 顯示圖形
plt.show()