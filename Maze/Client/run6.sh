#!/usr/bin/bash

./client 10000 -n ava -s Riemann &
./client 10000 -n bob -s Euclid &
./client 10000 -n charlie -s Euclid &
./client 10000 -n dan -s Riemann &
./client 10000 -n ellen -s Riemann &
./client 10000 -n fredrick -s Euclid
