#pc 3, a 0, b 1, c 2, e 4, f 5

a = 1
b = 0
c = 0
e = 0
f = 0

c = c + 2
c = c * c
c = 19 * c  # 19 == pc
c = c * 11
e = e + 2
e = 22 * e  # 22 = pc
e = e + 2
c = c + e
if a == 1:
  e = 27
  e = e * 28
  e = e + 29
  e = e * 30
  e = e * 14
  e = e * 32
  c = c + e
  a = 0

def fn1():
  global a
  global b
  global c
  global e
  global f
  while True:
    a = 1
    f = 1
    while True:
      if b * f == c:
        a = b + a
      f = f + 1
      if f > c:
        break
    b = b + 1
    if b > c:
      break

def fn2():
  global a
  global b
  global c
  global e
  global f
  b = 1
  while b <= c:
    x = c % b
    if x == 0:
      a = a + b
    b += 1

print a, b, c, e, f
fn2()
print a, b, c, e, f
