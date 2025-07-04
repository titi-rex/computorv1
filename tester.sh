#!/bin/bash

RESET="\e[0m"
RED="\e[0;31m"
GREEN_B="\e[1;32m"
YELLOW="\e[0;33m"
YELLOW_B="\e[1;33m"
BLUE="\e[0;34m"
PURPLE_B="\e[1;35m"

EXE=$1

if [[ -z "$EXE" ]]; then
    printf "$RED""No executable provided\n$RESET"
    exit
fi



print_exp() {
    printf "$YELLOW_B""\nExpected: \n"$YELLOW"Reduced form: $REDUCED\n"
    if [[ "$N_ROOTS" == 0 ]]; then
        printf "No solution.\n$RESET"
    elif [[ "$N_ROOTS" == "inf" ]]; then
        printf "Any real number is a solution.\n$RESET"
    elif [[ "$N_ROOTS" == 1 ]]; then
        printf "One Solution: $ROOT\n$RESET"
    elif [[ "$N_ROOTS" == 2 ]]; then
        printf "Two real solutions: $ROOT and $ROOT_2\n$RESET"
    elif [[ "$N_ROOTS" == "2i" ]]; then
        printf "Two complex solutions: $ROOT\n$RESET"
    else
        printf "Degree > 3, can't solve\n$RESET"
    fi
}

print_got() {
    printf "$PURPLE_B""\nGot: \n$RESET"
    $EXE "$EXPRESSION"
}


for filename in tests/*.test; do
    [ -e "$filename" ] || continue
    source "$filename"
    printf "$BLUE""\n#=====================\t$filename\t======================#\n$RESET"
    printf "$GREEN_B""Expression: $EXPRESSION\n$RESET"
    print_exp
    print_got
done

printf "$BLUE""\n#===============================================#\n$RESET"
