# This is a test WDL workflow
task hello {
  input {
    String name
    Int count = 3
  }

  command {
    echo "Hello ${name}" 
    echo "Count: ${count}"
  }

  output {
    File output_file = stdout()
  }
}

workflow test_workflow {
  input {
    String person_name = "World"
  }

  call hello {
    input:
      name = person_name
  }
}
