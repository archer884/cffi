int c_is_prime(unsigned long long int number) {
    if (number <= 1)
        return 0;

    if (number == 2)
        return 0;

    unsigned long long int i;

    for (i=3; i*i<=number; i+=2) {
        if (number % i == 0) return 0;
    }

    return 1;
}
