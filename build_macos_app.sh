#!/usr/bin/env bash
py2applet --make-setup pgame.py
rm -rf build dist
python setup.py py2app -A
#open -a dist/pgame.app
