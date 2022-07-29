function initState()
  local state = {
    tokens = nil,
    position = 0
  }
  return state
end

function next(state, arg1)
  return arg1
end

function peek(state, arg1)
  return arg1
end

function tokenize(input)
  local rx = "[\\s,]*(~@|[\\[\\]{}()'`~^@]|\"(?:\\\\.|[^\\\\\"])*\"?|;[^\n]*|[^\\s\\[\\]{}('\"`,;)]*)"
  local iterator = string.gmatch(input, rx)

  for token in string.gmatch(",,~@", "[%s,]*(~@)?") do
    print(token)
  end

  local tokens = {}
  local i = 1

  for token in iterator do
    tokens[i] = token
    i = i + 1
  end

  return tokens
end

function dump(o)
  if type(o) == 'table' then
    local s = '{ '
    for k, v in pairs(o) do
      if type(k) ~= 'number' then k = '"' .. k .. '"' end
      s = s .. '[' .. k .. '] = ' .. dump(v) .. ','
    end
    return s .. '} '
  else
    return tostring(o)
  end
end

tokenized = tokenize("(+ 1 2)")

print(dump(tokenized))
