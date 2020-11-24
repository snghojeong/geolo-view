cargo build

cp target/debug/libgeolo\_view.dylib ./geolo\_view.so

python geolo_viewer.py test.txt
