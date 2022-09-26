int x;

int proc(int z, int w) {
    x = 2;
    return z - w;
}

int main() {
    int y;
    y = proc(5, 2);
    return x + y;
}
