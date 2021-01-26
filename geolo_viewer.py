import geolo_view
import sys

ret = geolo_view.read_log(sys.argv[1], 0, int(sys.argv[2]), sys.argv[3], sys.argv[4], False)
print(ret)
