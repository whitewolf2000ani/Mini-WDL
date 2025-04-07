task hello {
  input {
    String name
  }
  command {
    echo "Hello ${name}"
  }
}
