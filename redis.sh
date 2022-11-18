#!/bin/bash

# Default to '*' key pattern, meaning all redis keys in the namespace
REDIS_KEY_PATTERN="${REDIS_KEY_PATTERN:-*}"
for key in $(redis-cli --scan --pattern "$REDIS_KEY_PATTERN")
do
    type=$(redis-cli type $key)
    if [ $type = "list" ]
    then
        printf "$key => \n$(redis-cli lrange $key 0 -1 | sed 's/^/  /')\n"
    elif [ $type = "hash" ]
    then
        printf "$key => \n$(redis-cli hgetall $key | sed 's/^/  /')\n"
    else
        printf "$key => $(redis-cli get $key)\n"
    fi
done