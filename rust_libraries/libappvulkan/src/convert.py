
import re


func = r"typedef (VkResult|void) VKAPI_PTR \*\(\w+\)"

file = open("./vulkan_core.rs","r+")
contents = file.read()

contents = contents.replace("const VkInstanceCreateInfo* pCreateInfo","p_create_info:*const VkInstanceCreateInfo")
contents = contents.replace("const VkAllocationCallbacks* pAllocator","p_allocator:*const VkAllocationCallbacks")
contents = contents.replace("const VkAllocationCallbacks* pAllocator","p_allocator:*const VkAllocationCallbacks")
contents = contents.replace("VkInstance* pInstance","p_instance:*mut VkInstance")


pattern = re.compile(r"typedef void \(VKAPI_PTR \*(\w+)\)\(([A-Za-z ,]+)\);")

matches = re.findall(pattern,contents)

functions = dict()

lines =contents.splitlines()
for index,line in enumerate(iterable=lines):
    match = pattern.findall(line)
    if len(match) == 0:
        continue
    if len(match) == 1:
        match = list(match[0])
    name = match[0]
    print(name)
    arguments = match[1].split(', ')
    newline = f"type {name} = unsafe extern \"system\" fn("
    for argument in arguments:
        split = argument.split(" ")
        left = split[0]
        right = split[1]
        left = left.strip()
        right = right.strip()

        newline = newline + f"{right}:{left},"
    newline = newline + ');//CONVERTED'
    lines[index]=newline


contents = '\n'.join(lines)


externs =re.compile(r"VKAPI_ATTR (VkResult|void) VKAPI_CALL (\w+)\(\n( *([\w*]+) *(\w+)\n)+")
str = ""
startline=0
endline = 0
index = 0
re.M = True
re.DOTALL = True
matches=re.findall(externs,contents)
for match in matches:
    print(match)










# for match in matches:
#     name = match[0]
#     print(name)
#     arguments = match[1].split(', ')
#     line = f"type {name} = unsafe extern \"system\" fn("
#     for argument in arguments:
#         split = argument.split(" ")
#         left = split[0]
#         right = split[1]
#         left = left.strip()
#         right = right.strip()

#         line = line + f"{right}:{left},"
#     line = line + ');'

#     contents = contents.replace(rf"typedef void \(VKAPI_PTR *{match[0]}\)\({match[1]}\);",line)
file.seek(0)
file.write(contents)
file.close()