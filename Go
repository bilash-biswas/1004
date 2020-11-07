package main

import "fmt"

func main() {
	var num1 int
	var num2 int
	fmt.Scanln(&num1)
	fmt.Scanln(&num2)
	fmt.Printf("PROD = %d\n", (num1 * num2))
}
