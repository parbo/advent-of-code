
e = 123
e = e & 456
e = e == 72 ? 1 : 0
pc = pc + 1
pc = 0
d = 65536
e = 14464005
do
  c = d & 255
  e = e + c
  e = e & 16777215
  e = e * 65899
  e = e & 16777215
  d = d / 256
while d <= 256
c = a == e ? 1 : 0
pc = pc + c
pc = pc
