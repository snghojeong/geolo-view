import geolo_view
import sys

ret = geolo_view.read_log(sys.argv[1], 0, 1, False)
print(sys.argv[1])
print(ret)
