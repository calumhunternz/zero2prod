#!/usr/bin/env bash
docker stop zero2prod_db
docker remove zero2prod_db
./scripts/init_db.sh