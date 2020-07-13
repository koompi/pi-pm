#!/bin/bash

yes | sudo pacman -Sy rustup;
rustup default stable;

cd fake-api/
npm i
cd ..
