import geolo_view
import sys
import argparse

parser = argparse.ArgumentParser(description='Log reader')
parser.add_argument('--file', required=True, help='log file')
parser.add_argument('--cnt', required=True, help='the number of log line')
parser.add_argument('--lv', required=False, help='log level to filter')
parser.add_argument('--md', required=False, help='module name to filter')

args = parser.parse_args()

if args.md and args.lv:
    ret = geolo_view.read_log(args.file, 0, int(args.cnt), False, lv=args.lv, md=args.md)
elif args.md:
    ret = geolo_view.read_log(args.file, 0, int(args.cnt), False, md=args.md)
elif args.lv:
    ret = geolo_view.read_log(args.file, 0, int(args.cnt), False, lv=args.lv)
else:
    ret = geolo_view.read_log(args.file, 0, int(args.cnt), False)

print(ret["log"])
print(ret["pos"])
