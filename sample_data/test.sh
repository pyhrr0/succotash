BIN=../target/debug/engine
[[ -x $BIN ]] || exit

for f in $(ls -1 *_in.csv); do
    cur_item=${f/_in.csv}

    diff <($BIN $f | tail -n+2 | sort) <(tail -n+2 ${f/_in/_out} | sort) && \
    echo "$cur_item succeeded."> /dev/stdout || \
    echo "$cur_item failed."> /dev/stderr
done
