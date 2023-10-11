from time import strftime
import datetime
import curses

def update(x, y):
    z = int(y/12)
    w = int(y%12)
    clock = [ "○" for i in range(28)]
    hour_positions = [3,4,5,6,12,13,14,15,21,22,23,24]
    minute_positions = [0,1,2,8,9,10,17,18,19,25,26]
    special_positions = [7,11,16,20]
    if (x/12 >= 1):
        clock[27] = "●"
    if not (x == 0):
        clock[hour_positions[x%12-1]] = "●"
    if  not (w == 0):
        clock[minute_positions[w-1]] = "●"
    if  not (z == 0):
        clock[special_positions[z-1]] = "●"

    line0 = str.format(' '.join(clock[0:3]))
    line1 = str.format(' '.join(clock[3:7]))
    line2 = str.format(' '.join(clock[7:12]))
    line3 = str.format(' '.join(clock[12:16]))
    line4 = str.format(' '.join(clock[16:21]))
    line5 = str.format(' '.join(clock[21:25]))
    line6 = str.format(' '.join(clock[25:28]))
    matrix = [line0,line1,line2,line3,line4,line5,line6]

    screen.clear()
    for i in range(7):
        screen.addstr((curses.LINES // 2)+i,(curses.COLS // 2 - len(matrix[i]) //2),matrix[i])
    screen.refresh()

screen = curses.initscr()
curses.curs_set(0) 
minute = None
while True:
    if datetime.datetime.now().minute != minute:
        update(int(strftime("%H")), int(strftime("%M")))
        minute = datetime.datetime.now().minute
