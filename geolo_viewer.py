import geolo_view
import sys
import argparse

parser = argparse.ArgumentParser(description='Log reader')
parser.add_argument('--file', required=True, help='log file')
parser.add_argument('--cnt', required=True, help='the number of log line')
parser.add_argument('--seq', required=False, help='seq number to filter')
parser.add_argument('--date', required=False, help='date to filter')
parser.add_argument('--lv', required=False, help='log level to filter')
parser.add_argument('--md', required=False, help='module name to filter')
parser.add_argument('--msg', required=False, help='message string to filter')

args = parser.parse_args()

prog_exit = 0
pos = 0
pos_list = list()
while prog_exit == 0:
    read_pos = pos
    if args.md and args.lv:
        ret = geolo_view.read_log(args.file, read_pos, int(args.cnt), lv=args.lv, md=args.md)
    elif args.md:
        ret = geolo_view.read_log(args.file, read_pos, int(args.cnt), md=args.md)
    elif args.lv:
        ret = geolo_view.read_log(args.file, read_pos, int(args.cnt), lv=args.lv)
    else:
        ret = geolo_view.read_log(args.file, read_pos, int(args.cnt))
    print(ret["log"])
    keyin = input('Exit(q), Forward(f), Backward(b):')
    if keyin == 'q':
        prog_exit = 1
    elif keyin == 'b':
        pos = pos_list.pop()
    elif keyin == 'f':
        pos_list.append(pos)
        pos = ret["pos"]
    elif ret["log"] == "":
        prog_exit = 1
