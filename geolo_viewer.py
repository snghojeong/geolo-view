import geolo_view
import sys
import argparse

parser = argparse.ArgumentParser(description='Log reader')
parser.add_argument('--file', required=True, help='log file')
parser.add_argument('--cnt', required=True, help='the number of log line')
parser.add_argument('--lv', required=False, help='log level to filter')
parser.add_argument('--md', required=False, help='module name to filter')

args = parser.parse_args()

lv_str = ""
if args.lv:
    lv_str = args.lv

md_str = ""
if args.md:
    md_str = args.md

ret = geolo_view.read_log(args.file, 0, int(args.cnt), lv_str, md_str, False)
print(ret)
