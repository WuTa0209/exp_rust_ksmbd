import re
import matplotlib.pyplot as plt

file_path = 'C_alloc.txt' 
file_path1 = 'Rust_alloc.txt'
total_time = 0
total_time1 = 0
C_time = []
Rust_time = []
with open(file_path, 'r') as file:
    lines = file.readlines()
with open(file_path1, 'r') as file:
    lines1 = file.readlines()

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

print("C Avg:", total_time/100)
# get standard deviation from C_time list
mean = sum(C_time) / len(C_time)
variance = sum([((x - mean) ** 2) for x in C_time]) / len(C_time)
res = variance ** 0.5
print("C Standard deviation:", res/100)


print("Rust Avg:", total_time1/100)
# get standard deviation from Rust_time list
mean = sum(Rust_time) / len(Rust_time)
variance = sum([((x - mean) ** 2) for x in Rust_time]) / len(Rust_time)
res = variance ** 0.5
print("Rust Standard deviation:", res/100)


x_coords1 = [i for i in range(100)]
y_coords1 = [i for i in C_time]
plt.plot(x_coords1, y_coords1, marker='o', linestyle='-', color='blue')

x_coords2 = [i for i in range(100)]
y_coords2 = [i for i in Rust_time]
plt.plot(x_coords2, y_coords2, marker='o', linestyle='-', color='orange')

# 添加標籤和標題
plt.xlabel('times')
plt.ylabel('time(us)')
plt.title('C vs Rust in alloc')

# 顯示圖形
plt.show()

