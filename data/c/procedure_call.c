int x;

int proc() {
    x = 2;
    return 3;
}

int main() {
    int y;
    y = proc();
    return x + y;
}
