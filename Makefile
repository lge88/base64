STD_BASE64=/usr/bin/base64
RUST_BASE64=target/release/base64
CPP_BASE64=csrc/base64

TEST_INPUT_DIR=test_input
TEST_OUTPUT_DIR=test_output

TEST_CASES=11m_no_padding.bin
TEST_CASES+=11m_pad_1.bin
TEST_CASES+=11m_pad_2.bin

exe:
	cargo build --release
	(cd csrc && g++ -O3 base64.cpp -o base64)

test_output:
	mkdir -p test_output
	rm -fr test_output/*
	cp $(TEST_INPUT_DIR)/* $(TEST_OUTPUT_DIR)

test: exe test_output
	$(foreach t,$(TEST_CASES),\
		$(STD_BASE64) <$(TEST_OUTPUT_DIR)/$(t) | fold -w 80 >$(TEST_OUTPUT_DIR)/std_$(t).base64; \
		$(RUST_BASE64) <$(TEST_OUTPUT_DIR)/$(t) | fold -w 80 >$(TEST_OUTPUT_DIR)/my_$(t).base64; \
		diff $(TEST_OUTPUT_DIR)/std_$(t).base64 $(TEST_OUTPUT_DIR)/my_$(t).base64; \
	)

random_input: test_output
	dd if=/dev/urandom of=$(TEST_OUTPUT_DIR)/random_bytes_100m.bin bs=512 count=204800

bench: random_input
	time $(RUST_BASE64) <$(TEST_OUTPUT_DIR)/random_bytes_100m.bin >$(TEST_OUTPUT_DIR)/my_random_bytes_100m.bin.base64
	time $(STD_BASE64) <$(TEST_OUTPUT_DIR)/random_bytes_100m.bin >$(TEST_OUTPUT_DIR)/std_random_bytes_100m.bin.base64
	time $(CPP_BASE64) <$(TEST_OUTPUT_DIR)/random_bytes_100m.bin >$(TEST_OUTPUT_DIR)/cpp_random_bytes_100m.bin.base64
	diff $(TEST_OUTPUT_DIR)/my_random_bytes_100m.bin.base64 $(TEST_OUTPUT_DIR)/std_random_bytes_100m.bin.base64
	diff $(TEST_OUTPUT_DIR)/cpp_random_bytes_100m.bin.base64 $(TEST_OUTPUT_DIR)/std_random_bytes_100m.bin.base64

.PHONY: test exe test_output random_input bench
