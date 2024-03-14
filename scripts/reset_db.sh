#!/usr/bin/env bash
docker stop zero2prod_db
docker remove zero2prod_db
./init_db.sh