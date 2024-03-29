use clap::{builder::EnumValueParser, ArgAction, Parser, Subcommand, ValueEnum};
use console::style;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::str::FromStr;

const DEFAULT_SOCKET_ADDRESS: &str = "0.0.0.0:8080";
const PROMPT_TEMPLATES: [&str; 20] = [
    "llama-2-chat",
    "mistral-instruct",
    "mistrallite",
    "codellama-instruct",
    "codellama-super-instruct",
    "openchat",
    "human-assistant",
    "vicuna-1.0-chat",
    "vicuna-1.1-chat",
    "vicuna-llava",
    "chatml",
    "baichuan-2",
    "wizard-coder",
    "zephyr",
    "stablelm-zephyr",
    "intel-neural",
    "deepseek-chat",
    "deepseek-coder",
    "solar-instruct",
    "gemma-instruct",
];

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(default_value = DEFAULT_SOCKET_ADDRESS, help = "Sets the socket address")]
    socket_addr: String,
    #[arg(long, help = "Print prompt strings to stdout")]
    log_prompts: bool,
    #[arg(long, help = "Print statistics to stdout")]
    log_stat: bool,
    #[arg(long, help = "Print all log information to stdout")]
    log_all: bool,
    #[arg(
        default_value = "chatbot-ui",
        long,
        help = "Path to the root directory of target Web UI"
    )]
    web_ui: String,
}

#[derive(Debug, Clone, Subcommand)]
enum Commands {
    Chat {
        #[arg(short = 'm', long = "model-name", help = "Sets the model name")]
        model_name: String,
        #[arg(
            short = 'a',
            long,
            help = "Sets the alias of the model used by WasmEdge Runtime",
            default_value = "default"
        )]
        model_alias: String,
        #[arg(
            short = 'c',
            long,
            help = "Sets the prompt context size",
            default_value = "512"
        )]
        ctx_size: u64,
        #[arg(short = 'p', long, help = "Sets the prompt template")]
        prompt_template: Option<String>,
        #[arg(short = 'r', long, help = "Halt generation at PROMPT, return control.")]
        reverse_prompt: Option<String>,
        #[arg(
            short = 'n',
            long,
            help = "Number of tokens to predict",
            default_value = "1024"
        )]
        n_predict: u64,
        #[arg(
            short = 'g',
            long,
            help = "Number of layers to run on the GPU",
            default_value = "100"
        )]
        n_gpu_layers: u64,
        #[arg(
            short = 'b',
            long,
            help = "Batch size for prompt processing",
            default_value = "512"
        )]
        batch_size: u64,
        #[arg(
            long = "temp",
            help = "Temperature for sampling",
            default_value = "0.8"
        )]
        temperature: f64,
        #[arg(
            long,
            help = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. 1.0 = disabled.",
            default_value = "0.9"
        )]
        top_p: f64,
        #[arg(
            long,
            help = "Penalize repeat sequence of tokens",
            default_value = "1.1"
        )]
        repeat_penalty: f64,
        #[arg(
            long,
            help = "Repeat alpha presence penalty. 0.0 = disabled",
            default_value = "0.0"
        )]
        presence_penalty: f64,
        #[arg(
            long,
            help = "Repeat alpha frequency penalty. 0.0 = disabled",
            default_value = "0.0"
        )]
        frequency_penalty: f64,
    },
    Llava {
        #[arg(short = 'm', long = "model-name", help = "Sets the model name")]
        model_name: String,
        #[arg(
            long = "projector-file",
            help = "Path to the multimodal projector file"
        )]
        projector: String,
        #[arg(
            short = 'a',
            long,
            help = "Sets the alias of the model used by WasmEdge Runtime",
            default_value = "default"
        )]
        model_alias: String,
        #[arg(
            short = 'c',
            long,
            help = "Sets the prompt context size",
            default_value = "512"
        )]
        ctx_size: u64,
        #[arg(short = 'p', long, help = "Sets the prompt template")]
        prompt_template: Option<String>,
        #[arg(short = 'r', long, help = "Halt generation at PROMPT, return control.")]
        reverse_prompt: Option<String>,
        #[arg(
            short = 'n',
            long,
            help = "Number of tokens to predict",
            default_value = "1024"
        )]
        n_predict: u64,
        #[arg(
            short = 'g',
            long,
            help = "Number of layers to run on the GPU",
            default_value = "100"
        )]
        n_gpu_layers: u64,
        #[arg(
            short = 'b',
            long,
            help = "Batch size for prompt processing",
            default_value = "512"
        )]
        batch_size: u64,
        #[arg(
            long = "temp",
            help = "Temperature for sampling",
            default_value = "0.8"
        )]
        temperature: f64,
        #[arg(
            long,
            help = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. 1.0 = disabled.",
            default_value = "0.9"
        )]
        top_p: f64,
        #[arg(
            long,
            help = "Penalize repeat sequence of tokens",
            default_value = "1.1"
        )]
        repeat_penalty: f64,
        #[arg(
            long,
            help = "Repeat alpha presence penalty. 0.0 = disabled",
            default_value = "0.0"
        )]
        presence_penalty: f64,
        #[arg(
            long,
            help = "Repeat alpha frequency penalty. 0.0 = disabled",
            default_value = "0.0"
        )]
        frequency_penalty: f64,
    },
    Embedding {
        #[arg(short = 'm', long = "model-name", help = "Sets the model name")]
        model_name: String,
        #[arg(
            short = 'a',
            long,
            help = "Sets the alias of the model used by WasmEdge Runtime",
            default_value = "embedding"
        )]
        model_alias: String,
        #[arg(short = 'c', long, help = "Sets the prompt context size")]
        ctx_size: u64,
    },
    Rag {
        #[arg(
            short = 'm',
            long = "model-name",
            value_delimiter = ',',
            help = "Set the names for both chat and embedding models, for example, '--model-name llama-2-7b,all-MiniLM-L6-v2'"
        )]
        model_name: Vec<String>,
        #[arg(
            short = 'a',
            long,
            value_delimiter = ',',
            help = "Set the alias of both chat and embedding models used by WasmEdge Runtime",
            default_value = "default,embedding"
        )]
        model_alias: Vec<String>,
        #[arg(
            short = 'c',
            long,
            help = "Set context sizes for both chat and embedding models, for example, '--ctx-size 4096,384'"
        )]
        ctx_size: Vec<u64>,
        #[arg(
            short = 'p',
            long,
            help = "Set the prompt template. This is for chat model."
        )]
        prompt_template: Option<String>,
        #[arg(
            short = 'r',
            long,
            help = "Set the reverse prompt This is for chat model.if needed. This is for chat model."
        )]
        reverse_prompt: Option<String>,
        #[arg(
            short = 'n',
            long,
            help = "Number of tokens to predict. This is for chat model.",
            default_value = "1024"
        )]
        n_predict: u64,
        #[arg(
            short = 'g',
            long,
            help = "Number of layers to run on the GPU. This is for chat model.",
            default_value = "100"
        )]
        n_gpu_layers: u64,
        #[arg(
            short = 'b',
            long,
            help = "Batch size for prompt processing. This is for chat model.",
            default_value = "512"
        )]
        batch_size: u64,
        #[arg(
            long = "temp",
            help = "Temperature for sampling. This is for chat model.",
            default_value = "0.8"
        )]
        temperature: f64,
        #[arg(
            long,
            help = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. 1.0 = disabled. This is for chat model.",
            default_value = "0.9"
        )]
        top_p: f64,
        #[arg(
            long,
            help = "Penalize repeat sequence of tokens. This is for chat model.",
            default_value = "1.1"
        )]
        repeat_penalty: f64,
        #[arg(
            long,
            help = "Repeat alpha presence penalty. 0.0 = disabled. This is for chat model.",
            default_value = "0.0"
        )]
        presence_penalty: f64,
        #[arg(
            long,
            help = "Repeat alpha frequency penalty. 0.0 = disabled. This is for chat model.",
            default_value = "0.0"
        )]
        frequency_penalty: f64,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Chat {
            model_name,
            model_alias,
            ctx_size,
            prompt_template,
            reverse_prompt,
            n_predict,
            n_gpu_layers,
            batch_size,
            temperature,
            top_p,
            repeat_penalty,
            presence_penalty,
            frequency_penalty,
        } => chat_command_params(
            model_name,
            model_alias,
            ctx_size,
            prompt_template,
            reverse_prompt,
            n_predict,
            n_gpu_layers,
            batch_size,
            temperature,
            top_p,
            repeat_penalty,
            presence_penalty,
            frequency_penalty,
        )?,
        Commands::Llava {
            model_name,
            projector,
            model_alias,
            ctx_size,
            prompt_template,
            reverse_prompt,
            n_predict,
            n_gpu_layers,
            batch_size,
            temperature,
            top_p,
            repeat_penalty,
            presence_penalty,
            frequency_penalty,
        } => unimplemented!(),
        Commands::Embedding {
            model_name,
            model_alias,
            ctx_size,
        } => unimplemented!(),
        Commands::Rag {
            model_name,
            model_alias,
            ctx_size,
            prompt_template,
            reverse_prompt,
            n_predict,
            n_gpu_layers,
            batch_size,
            temperature,
            top_p,
            repeat_penalty,
            presence_penalty,
            frequency_penalty,
        } => unimplemented!(),
    }

    Ok(())
}

fn chat_command_params<S: AsRef<str>>(
    model_name: S,
    model_alias: S,
    ctx_size: u64,
    prompt_template: Option<String>,
    reverse_prompt: Option<String>,
    n_predict: u64,
    n_gpu_layers: u64,
    batch_size: u64,
    temperature: f64,
    top_p: f64,
    repeat_penalty: f64,
    presence_penalty: f64,
    frequency_penalty: f64,
) -> anyhow::Result<()> {
    let template = match prompt_template {
        Some(pt) => match PromptTemplateType::from_str(&pt) {
            Ok(t) => t,
            Err(_) => {
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        "The given prompt template is invalid. Please select from the following:",
                    )
                    .default(0)
                    .items(&PROMPT_TEMPLATES[..])
                    .interact_on_opt(&Term::stderr())?;

                match selection {
                    Some(0) => PromptTemplateType::Llama2Chat,
                    Some(1) => PromptTemplateType::MistralInstruct,
                    _ => panic!("Fatal: No selection!"),
                }
            }
        },
        None => {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Please select a prompt template from the following:")
                .default(0)
                .items(&PROMPT_TEMPLATES[..])
                .interact_on_opt(&Term::stderr())?;

            match selection {
                Some(0) => PromptTemplateType::Llama2Chat,
                Some(1) => PromptTemplateType::MistralInstruct,
                _ => panic!("Fatal: No selection!"),
            }
        }
    };

    println!("\n\n");
    println!(
        "[INFO] Model Name: {:?}",
        style(model_name.as_ref()).green()
    );
    println!(
        "[INFO] Model Alias: {:?}",
        style(model_alias.as_ref()).green()
    );
    println!("[INFO] Context Size: {:?}", style(ctx_size).green());
    println!("[INFO] Prompt Template: {:?}", style(template).green());
    println!("[INFO] Reverse Prompt: {:?}", style(reverse_prompt).green());
    println!(
        "[INFO] Number of Tokens to Predict: {:?}",
        style(n_predict).green()
    );
    println!(
        "[INFO] Number of Layers to Run on the GPU: {:?}",
        style(n_gpu_layers).green()
    );
    println!("[INFO] Batch Size: {:?}", style(batch_size).green());
    println!("[INFO] Temperature: {:?}", style(temperature).green());
    println!("[INFO] Top P: {:?}", style(top_p).green());
    println!("[INFO] Repeat Penalty: {:?}", style(repeat_penalty).green());
    println!(
        "[INFO] Presence Penalty: {:?}",
        style(presence_penalty).green()
    );
    println!(
        "[INFO] Frequency Penalty: {:?}",
        style(frequency_penalty).green()
    );

    Ok(())
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum PromptTemplateType {
    Llama2Chat,
    MistralInstruct,
    MistralLite,
    OpenChat,
    CodeLlama,
    CodeLlamaSuper,
    HumanAssistant,
    VicunaChat,
    Vicuna11Chat,
    VicunaLlava,
    ChatML,
    Baichuan2,
    WizardCoder,
    Zephyr,
    StableLMZephyr,
    IntelNeural,
    DeepseekChat,
    DeepseekCoder,
    SolarInstruct,
    Phi2Chat,
    Phi2Instruct,
    GemmaInstruct,
}
impl FromStr for PromptTemplateType {
    type Err = String;

    fn from_str(template: &str) -> std::result::Result<Self, Self::Err> {
        match template {
            "llama-2-chat" => Ok(PromptTemplateType::Llama2Chat),
            "mistral-instruct" => Ok(PromptTemplateType::MistralInstruct),
            "mistrallite" => Ok(PromptTemplateType::MistralLite),
            "codellama-instruct" => Ok(PromptTemplateType::CodeLlama),
            "codellama-super-instruct" => Ok(PromptTemplateType::CodeLlamaSuper),
            "belle-llama-2-chat" => Ok(PromptTemplateType::HumanAssistant),
            "human-assistant" => Ok(PromptTemplateType::HumanAssistant),
            "vicuna-1.0-chat" => Ok(PromptTemplateType::VicunaChat),
            "vicuna-1.1-chat" => Ok(PromptTemplateType::Vicuna11Chat),
            "vicuna-llava" => Ok(PromptTemplateType::VicunaLlava),
            "chatml" => Ok(PromptTemplateType::ChatML),
            "openchat" => Ok(PromptTemplateType::OpenChat),
            "baichuan-2" => Ok(PromptTemplateType::Baichuan2),
            "wizard-coder" => Ok(PromptTemplateType::WizardCoder),
            "zephyr" => Ok(PromptTemplateType::Zephyr),
            "stablelm-zephyr" => Ok(PromptTemplateType::StableLMZephyr),
            "intel-neural" => Ok(PromptTemplateType::IntelNeural),
            "deepseek-chat" => Ok(PromptTemplateType::DeepseekChat),
            "deepseek-coder" => Ok(PromptTemplateType::DeepseekCoder),
            "solar-instruct" => Ok(PromptTemplateType::SolarInstruct),
            "phi-2-chat" => Ok(PromptTemplateType::Phi2Chat),
            "phi-2-instruct" => Ok(PromptTemplateType::Phi2Instruct),
            "gemma-instruct" => Ok(PromptTemplateType::GemmaInstruct),
            _ => Err(format!("Invalid prompt template: {}", template)),
        }
    }
}
impl std::fmt::Display for PromptTemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromptTemplateType::Llama2Chat => write!(f, "llama-2-chat"),
            PromptTemplateType::MistralInstruct => write!(f, "mistral-instruct"),
            PromptTemplateType::MistralLite => write!(f, "mistrallite"),
            PromptTemplateType::OpenChat => write!(f, "openchat"),
            PromptTemplateType::CodeLlama => write!(f, "codellama-instruct"),
            PromptTemplateType::HumanAssistant => write!(f, "human-asistant"),
            PromptTemplateType::VicunaChat => write!(f, "vicuna-1.0-chat"),
            PromptTemplateType::Vicuna11Chat => write!(f, "vicuna-1.1-chat"),
            PromptTemplateType::VicunaLlava => write!(f, "vicuna-llava"),
            PromptTemplateType::ChatML => write!(f, "chatml"),
            PromptTemplateType::Baichuan2 => write!(f, "baichuan-2"),
            PromptTemplateType::WizardCoder => write!(f, "wizard-coder"),
            PromptTemplateType::Zephyr => write!(f, "zephyr"),
            PromptTemplateType::StableLMZephyr => write!(f, "stablelm-zephyr"),
            PromptTemplateType::IntelNeural => write!(f, "intel-neural"),
            PromptTemplateType::DeepseekChat => write!(f, "deepseek-chat"),
            PromptTemplateType::DeepseekCoder => write!(f, "deepseek-coder"),
            PromptTemplateType::SolarInstruct => write!(f, "solar-instruct"),
            PromptTemplateType::Phi2Chat => write!(f, "phi-2-chat"),
            PromptTemplateType::Phi2Instruct => write!(f, "phi-2-instruct"),
            PromptTemplateType::CodeLlamaSuper => write!(f, "codellama-super-instruct"),
            PromptTemplateType::GemmaInstruct => write!(f, "gemma-instruct"),
        }
    }
}
