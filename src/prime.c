int c_is_prime(unsigned long long int number) {
    if (number <= 1)
        return 0;

    unsigned long long int i;

    for (i=2; i*i<=number; i++) {
        if (number % i == 0) return 0;
    }

    return 1;
}
