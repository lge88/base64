# TEST_INPUT_DIR='/Users/lige/.m2/repository/com/cenqua/clover/clover/3.0.2/clover-3.0.2.jar'
STD_BASE64=/usr/bin/base64
MY_BASE64=target/release/base64
TEST_INPUT_DIR=test_input
TEST_OUTPUT_DIR=test_output
TEST_CASES=11m_no_padding.bin
TEST_CASES+=11m_pad_1.bin
TEST_CASES+=11m_pad_2.bin

test: exe test_output
	$(foreach t,$(TEST_CASES),\
		$(STD_BASE64) <$(TEST_OUTPUT_DIR)/$(t) | fold -w 80 >$(TEST_OUTPUT_DIR)/std_$(t).base64; \
		$(MY_BASE64) <$(TEST_OUTPUT_DIR)/$(t) | fold -w 80 >$(TEST_OUTPUT_DIR)/my_$(t).base64; \
		diff $(TEST_OUTPUT_DIR)/std_$(t).base64 $(TEST_OUTPUT_DIR)/my_$(t).base64; \
	)

test_output:
	mkdir -p test_output
	rm -fr test_output/*
	cp $(TEST_INPUT_DIR)/* $(TEST_OUTPUT_DIR)

exe:
	cargo build --release

.PHONY: test exe test_output
