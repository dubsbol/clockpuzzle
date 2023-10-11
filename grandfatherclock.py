import datetime
import math

w = datetime.datetime.now()
x = int(w.strftime("%I"))
y = int(w.strftime("%M"))
k = (w.strftime("%M"))
z = (w.strftime("%p"))
print(x,":",k,z)

e = math.trunc(y//12)
i = y%12
if z == ("PM"):
    j = 0
else:
    j = 1
    
list0 = [0,0,0]
list1 = [0,0,0,0]
list2 = [0,0,0,0,0]
list3 = [0,0,0,0]
list4 = [0,0,0,0,0]
list5 = [0,0,0,0]
list6 = [0,0,0]

if j == 1:
    list6[2] = 1

if e == 1:
    list2[0] = 1
elif e == 2:
    list2[4] = 1
elif e == 3:
    list4[0] = 1
elif e == 4:
    list4[4] = 1

if i == 1:
    list0[0] = 1
elif i == 2:
    list0[1] = 1
elif i == 3:
    list0[2] = 1
elif i == 4:
    list2[1] = 1
elif i == 5:
    list2[2] = 1
elif i == 6:
    list2[3] = 1
elif i == 7:
    list4[1] = 1
elif i == 8:
    list4[2] = 1
elif i == 9:
    list4[3] = 1
elif i == 10:
    list6[1] = 1
elif i == 11:
    list6[2] = 1

if x == 1:
    list1[0] = 1
elif x == 2:
    list1[1] = 1
elif x == 3:
    list1[2] = 1
elif x == 4:
    list1[3] = 1
elif x == 5:
    list3[0] = 1
elif x == 6:
    list3[1] = 1
elif x == 7:
    list3[2] = 1
elif x == 8:
    list3[3] = 1
elif x == 9:
    list5[0] = 1
elif x == 10:
    list5[1] = 1
elif x == 11:
    list5[2] = 1
elif x == 12:
    list5[3] = 1




    


print("  ",list0)
print(" ",list1)
print(list2)
print(" ",list3)
print(list4)
print(" ",list5)
print("  ",list6)
    
    





    
    
