#!/usr/bin/python3
# Copyright 2021-2024 Simon McVittie
# SPDX-License-Identifier: CC0-1.0

import base64
import sys
from pathlib import Path
from typing import Iterable


def iter_output_dirs() -> Iterable[Path]:
    for librsvg in Path('target/release/build').glob('librsvg-*'):
        yield librsvg / 'out'

    yield Path('tests/output')


if __name__ == '__main__':
    for d in iter_output_dirs():
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
