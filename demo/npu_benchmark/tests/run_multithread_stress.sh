#!/bin/sh

BIN="./rknpu_multithread_submit"
TOTAL=50
i=1

if [ ! -f "$BIN" ]; then
    echo "ERROR: cannot find $BIN"
    exit 127
fi

while [ "$i" -le "$TOTAL" ]; do
    "$BIN" \
        --threads 3 \
        --warmup 0 \
        --rounds 1 >/dev/null 2>&1

    rc=$?

    if [ "$rc" -ne 0 ]; then
        echo "FAIL run=$i rc=$rc"
        exit "$rc"
    fi

    echo "PASS run=$i/$TOTAL"
    i=$((i + 1))
done

echo "ALL PASS: $TOTAL runs completed"
exit 0
