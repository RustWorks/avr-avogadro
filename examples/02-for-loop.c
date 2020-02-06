// We add volatile to prevent optimizations
int main () {
	volatile unsigned char a = 42;
	for(unsigned char i = 0; i < 100; ++i) {
		a = a + i;
	}
	return a;
}
