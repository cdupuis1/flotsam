//
// viginere.go
//
// Implement basic viginere cipher encrypt and decrypt
//
package main

import "fmt"

// Each character in a string is shifted by the current index of this key
const key = "abcdefghij"

func encrypt(text *string, key []byte) {
	textSlice := []byte(*text)
	keyIndex := 0
	keyLen := len(key)

	// Iterate over each byte of text and shift it up by the numeric value of
	// the character key[keyIndex]
	for i := 0; i < len(textSlice); i++ {
		textSlice[i] += key[keyIndex] % 255

		// Loop the key index back to 0 if we hit the end of the key
		if keyIndex > keyLen {
			keyIndex = 0
		}
	}

	*text = string(textSlice)
}

func decrypt(text *string, key []byte) {
	textSlice := []byte(*text)
	keyIndex := 0
	keyLen := len(key)

	// Iterate over each byte of text and shift it up by the numeric value of
	// the character key[keyIndex]
	for i := 0; i < len(textSlice); i++ {
		textSlice[i] -= key[keyIndex] % 255

		// Loop the key index back to 0 if we hit the end of the key
		if keyIndex > keyLen {
			keyIndex = 0
		}
	}

	*text = string(textSlice)
}

func main() {
	text := "The lazy fox jumped over the brown fence"
	keySlice := []byte(key)

	encrypt(&text, keySlice)
	fmt.Println("Encrypted text: " + text)

	decrypt(&text, keySlice)
	fmt.Println("Decrypted text: " + text)
}
