import sys

f = open(sys.argv[1], "rb")

contents = f.read()

nums = contents.decode("utf-8")

print(nums)
