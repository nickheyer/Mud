
import completionsData from './stdlib.json';

const words = {};
const commonAtoms = ["true", "false"];
const commonKeywords = completionsData.completions
  .filter(completion => completion.type === "keyword")
  .map(completion => completion.label);

const commonCommands = completionsData.completions
  .filter(completion => completion.type === "function")
  .map(completion => completion.label);


function define(style, dict) {
  for (var i = 0; i < dict.length; i++) {
    words[dict[i]] = style;
  }
}
define('atom', commonAtoms);
define('keyword', commonKeywords);
define('builtin', commonCommands);

function tokenBase(stream, state) {
  if (stream.eatSpace()) {
    return null;
  }

  const sol = stream.sol();
  const ch = stream.next();
  switch (ch) {
    case '\\':
      stream.next();
      return null;
  
    case '"':
      state.tokens.unshift(tokenString(ch, ch === "`" ? "quote" : "string"));
      return tokenize(stream, state);
  
    case '#':
      if (sol && stream.eat('!')) {
        stream.skipToEnd();
        return 'meta'; // 'comment'?
      }
      stream.skipToEnd();
      return 'comment';
  
    case '$':
      state.tokens.unshift(tokenDollar);
      return tokenize(stream, state);
  
    case '+':
    case '=':
      return 'operator';
  
    case '-':
      stream.eat('-');
      stream.eatWhile(/\w/);
      return 'attribute';
  
    case '<':
      if (stream.match("<<")) return "operator";
      var heredoc = stream.match(/^<-?\s*(?:['"]([^'"]*)['"]|([^'"\s]*))/);
      if (heredoc) {
        state.tokens.unshift(tokenHeredoc(heredoc[1] || heredoc[2]));
        return 'string.special';
      }
      break;
  
    default:
      if (/\d/.test(ch)) {
        stream.eatWhile(/\d/);
        if (stream.eol() || !/\w/.test(stream.peek())) {
          return 'number';
        }
      }
  }
  
  stream.eatWhile(/[\w-]/);
  const cur = stream.current();
  if (stream.peek() === '=' && /\w+/.test(cur)) {
    return 'def'
  };

  return words.hasOwnProperty(cur) ? words[cur] : null;
}

function tokenString(quote, style) {
  const close = quote == "(" ? ")" : quote == "{" ? "}" : quote;
  return function (stream, state) {
    let next, escaped = false;
    while ((next = stream.next()) != null) {
      if (next === close && !escaped) {
        state.tokens.shift();
        break;
      } else if (next === '$' && !escaped && quote !== "\"" && stream.peek() != close) {
        escaped = true;
        stream.backUp(1);
        state.tokens.unshift(tokenDollar);
        break;
      } else if (!escaped && quote !== close && next === quote) {
        state.tokens.unshift(tokenString(quote, style));
        return tokenize(stream, state);
      } else if (!escaped && /["]/.test(next) && !/["]/.test(quote)) {
        state.tokens.unshift(tokenStringStart(next, "string"));
        stream.backUp(1);
        break;
      }
      escaped = !escaped && next === '\\';
    }
    return style;
  };
}

function tokenStringStart(quote, style) {
  return function (stream, state) {
    state.tokens[0] = tokenString(quote, style);
    stream.next();
    return tokenize(stream, state);
  };
}

const tokenDollar = function (stream, state) {
  if (state.tokens.length > 1) {
    stream.eat('$');
  }

  const ch = stream.next();
  if (/['"({]/.test(ch)) {
    state.tokens[0] = tokenString(ch, ch == "(" ? "quote" : ch == "{" ? "def" : "string");
    return tokenize(stream, state);
  }
  if (!/\d/.test(ch)) {
    stream.eatWhile(/\w/);
  }

  state.tokens.shift();
  return 'def';
};

function tokenHeredoc(delim) {
  return function (stream, state) {
    if (stream.sol() && stream.string == delim) {
      state.tokens.shift();
    }
    stream.skipToEnd();
    return "string.special";
  };
}

function tokenize(stream, state) {
  return (state.tokens[0] || tokenBase)(stream, state);
}

export const mud = {
  name: "mud",
  startState: function () { return { tokens: [] }; },
  token: function (stream, state) {
    return tokenize(stream, state);
  },
  languageData: {
    autocomplete: completionsData.completions,
    closeBrackets: { brackets: ["(", "[", "{", '"'] },
    commentTokens: { line: "#" }
  }
};
