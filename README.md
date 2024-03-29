# PROTOTYPE OF NEW SERVER CLI

## General

```bash
$ wasmedge server-cli-2.wasm --help

Usage: server-cli-2.wasm [OPTIONS] [SOCKET_ADDR] <COMMAND>

Commands:
  chat
  llava
  embedding
  rag
  help       Print this message or the help of the given subcommand(s)

Arguments:
  [SOCKET_ADDR]  Sets the socket address [default: 0.0.0.0:8080]

Options:
      --log-prompts      Print prompt strings to stdout
      --log-stat         Print statistics to stdout
      --log-all          Print all log information to stdout
      --web-ui <WEB_UI>  Path to the root directory of target Web UI [default: chatbot-ui]
  -h, --help             Print help
  -V, --version          Print version
```

## Chat Subcommand

```bash
$ wasmedge server-cli-2.wasm chat --help

Usage: server-cli-2.wasm chat [OPTIONS] --model-name <MODEL_NAME>

Options:
  -m, --model-name <MODEL_NAME>
          Sets the model name
  -a, --model-alias <MODEL_ALIAS>
          Sets the alias of the model used by WasmEdge Runtime [default: default]
  -c, --ctx-size <CTX_SIZE>
          Sets the prompt context size [default: 512]
  -p, --prompt-template <PROMPT_TEMPLATE>
          Sets the prompt template
  -r, --reverse-prompt <REVERSE_PROMPT>
          Halt generation at PROMPT, return control.
  -n, --n-predict <N_PREDICT>
          Number of tokens to predict [default: 1024]
  -g, --n-gpu-layers <N_GPU_LAYERS>
          Number of layers to run on the GPU [default: 100]
  -b, --batch-size <BATCH_SIZE>
          Batch size for prompt processing [default: 512]
      --temp <TEMPERATURE>
          Temperature for sampling [default: 0.8]
      --top-p <TOP_P>
          An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. 1.0 = disabled. [default: 0.9]
      --repeat-penalty <REPEAT_PENALTY>
          Penalize repeat sequence of tokens [default: 1.1]
      --presence-penalty <PRESENCE_PENALTY>
          Repeat alpha presence penalty. 0.0 = disabled [default: 0.0]
      --frequency-penalty <FREQUENCY_PENALTY>
          Repeat alpha frequency penalty. 0.0 = disabled [default: 0.0]
  -h, --help
          Print help
```

## Embedding Subcommand

```bash
$ wasmedge server-cli-2.wasm embedding --help

Usage: server-cli-2.wasm embedding [OPTIONS] --model-name <MODEL_NAME> --ctx-size <CTX_SIZE>

Options:
  -m, --model-name <MODEL_NAME>    Sets the model name
  -a, --model-alias <MODEL_ALIAS>  Sets the alias of the model used by WasmEdge Runtime [default: embedding]
  -c, --ctx-size <CTX_SIZE>        Sets the prompt context size
  -h, --help                       Print help
```

## RAG Subcommand

```bash
$ wasmedge server-cli-2.wasm rag --help

Usage: server-cli-2.wasm rag [OPTIONS] --qdrant-url <QDRANT_URL>

Options:
  -m, --model-name <MODEL_NAME>
          Set the names for both chat and embedding models, for example, '--model-name llama-2-7b,all-MiniLM-L6-v2'
  -a, --model-alias <MODEL_ALIAS>
          Set the alias of both chat and embedding models used by WasmEdge Runtime [default: default,embedding]
  -c, --ctx-size <CTX_SIZE>
          Set context sizes for both chat and embedding models, for example, '--ctx-size 4096,384'
  -p, --prompt-template <PROMPT_TEMPLATE>
          Set the prompt template. This is for chat model.
  -r, --reverse-prompt <REVERSE_PROMPT>
          Set the reverse prompt This is for chat model.if needed. This is for chat model.
  -n, --n-predict <N_PREDICT>
          Number of tokens to predict. This is for chat model. [default: 1024]
  -g, --n-gpu-layers <N_GPU_LAYERS>
          Number of layers to run on the GPU. This is for chat model. [default: 100]
  -b, --batch-size <BATCH_SIZE>
          Batch size for prompt processing. This is for chat model. [default: 512]
      --temp <TEMPERATURE>
          Temperature for sampling. This is for chat model. [default: 0.8]
      --top-p <TOP_P>
          An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. 1.0 = disabled. This is for chat model. [default: 0.9]
      --repeat-penalty <REPEAT_PENALTY>
          Penalize repeat sequence of tokens. This is for chat model. [default: 1.1]
      --presence-penalty <PRESENCE_PENALTY>
          Repeat alpha presence penalty. 0.0 = disabled. This is for chat model. [default: 0.0]
      --frequency-penalty <FREQUENCY_PENALTY>
          Repeat alpha frequency penalty. 0.0 = disabled. This is for chat model. [default: 0.0]
      --qdrant-url <QDRANT_URL>
          Sets the url of Qdrant REST Service (e.g., http://0.0.0.0:6333).
      --qdrant-collection-name <QDRANT_COLLECTION_NAME>
          Sets the collection name of Qdrant. Required for RAG. [default: default]
      --qdrant-limit <QDRANT_LIMIT>
          Max number of retrieved result. [default: 3]
      --qdrant-score-threshold <QDRANT_SCORE_THRESHOLD>
          Minimal score threshold for the search result [default: 0.0]
  -h, --help
          Print help
```

## LLAVA Subcommand

```bash
$ wasmedge server-cli-2.wasm llava --help

Usage: server-cli-2.wasm llava [OPTIONS] --model-name <MODEL_NAME> --projector-file <PROJECTOR>

Options:
  -m, --model-name <MODEL_NAME>
          Sets the model name
      --projector-file <PROJECTOR>
          Path to the multimodal projector file
  -a, --model-alias <MODEL_ALIAS>
          Sets the alias of the model used by WasmEdge Runtime [default: default]
  -c, --ctx-size <CTX_SIZE>
          Sets the prompt context size [default: 512]
  -p, --prompt-template <PROMPT_TEMPLATE>
          Sets the prompt template
  -r, --reverse-prompt <REVERSE_PROMPT>
          Halt generation at PROMPT, return control.
  -n, --n-predict <N_PREDICT>
          Number of tokens to predict [default: 1024]
  -g, --n-gpu-layers <N_GPU_LAYERS>
          Number of layers to run on the GPU [default: 100]
  -b, --batch-size <BATCH_SIZE>
          Batch size for prompt processing [default: 512]
      --temp <TEMPERATURE>
          Temperature for sampling [default: 0.8]
      --top-p <TOP_P>
          An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. 1.0 = disabled. [default: 0.9]
      --repeat-penalty <REPEAT_PENALTY>
          Penalize repeat sequence of tokens [default: 1.1]
      --presence-penalty <PRESENCE_PENALTY>
          Repeat alpha presence penalty. 0.0 = disabled [default: 0.0]
      --frequency-penalty <FREQUENCY_PENALTY>
          Repeat alpha frequency penalty. 0.0 = disabled [default: 0.0]
  -h, --help
          Print help
```
