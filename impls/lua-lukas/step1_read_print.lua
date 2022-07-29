function READ(arg1)
  return arg1
end

function EVAL(arg1)
  return arg1
end

function PRINT(arg1)
  return arg1
end

function rep(arg1)
  return PRINT(EVAL(READ(arg1)))
end


run = true
while (run) do
  print "user> "
  input = io.stdin:read()
  if (input == nil)  then
    run = false
  else
    print(rep(input))
  end
end
