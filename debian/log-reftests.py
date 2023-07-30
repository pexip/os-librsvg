#!/usr/bin/python3
# Copyright 2021-2022 Simon McVittie
# SPDX-License-Identifier: CC0-1.0

import base64
import sys
from pathlib import Path

if __name__ == '__main__':
    for librsvg in Path('target/release/build').glob('librsvg-*'):
        d = librsvg / 'out'
        for diff in d.glob('*-diff.png'):
            out = Path(str(diff)[:-len('-diff.png')] + '-out.png')

            for path in (out, diff):
                if path.exists():
                    print('')
                    print('begin-base64 644 %s' % path)
                    sys.stdout.flush()
                    with open(path, 'rb') as reader:
                        base64.encode(reader, sys.stdout.buffer)
                    print('====')
                    print('')

            print('')
