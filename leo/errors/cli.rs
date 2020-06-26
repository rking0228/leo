use crate::errors::*;

#[derive(Debug, Error)]
pub enum CLIError {
    #[error("{}", _0)]
    BuildError(BuildError),

    #[error("{}", _0)]
    BytesFileError(ZipFileError),

    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("{}", _0)]
    ChecksumFileError(ChecksumFileError),

    #[error("{}", _0)]
    GitignoreError(GitignoreError),

    #[error("{}", _0)]
    InitError(InitError),

    #[error("{}", _0)]
    InputsDirectoryError(InputsDirectoryError),

    #[error("{}", _0)]
    InputsFileError(InputsFileError),

    #[error("{}", _0)]
    MainFileError(MainFileError),

    #[error("{}", _0)]
    ManifestError(ManifestError),

    #[error("{}", _0)]
    NewError(NewError),

    #[error("{}", _0)]
    OutputsDirectoryError(OutputsDirectoryError),

    #[error("{}", _0)]
    ProofFileError(ProofFileError),

    #[error("{}", _0)]
    ProvingKeyFileError(ProvingKeyFileError),

    #[error("{}", _0)]
    RunError(RunError),

    #[error("{}", _0)]
    SourceDirectoryError(SourceDirectoryError),

    #[error("{}", _0)]
    TestError(TestError),

    #[error("{}", _0)]
    VerificationKeyFileError(VerificationKeyFileError),
}

impl From<ZipFileError> for CLIError {
    fn from(error: ZipFileError) -> Self {
        log::error!("{}\n", error);
        CLIError::BytesFileError(error)
    }
}

impl From<BuildError> for CLIError {
    fn from(error: BuildError) -> Self {
        log::error!("{}\n", error);
        CLIError::BuildError(error)
    }
}

impl From<ChecksumFileError> for CLIError {
    fn from(error: ChecksumFileError) -> Self {
        log::error!("{}\n", error);
        CLIError::ChecksumFileError(error)
    }
}

impl From<GitignoreError> for CLIError {
    fn from(error: GitignoreError) -> Self {
        log::error!("{}\n", error);
        CLIError::GitignoreError(error)
    }
}

impl From<InitError> for CLIError {
    fn from(error: InitError) -> Self {
        log::error!("{}\n", error);
        CLIError::InitError(error)
    }
}

impl From<InputsDirectoryError> for CLIError {
    fn from(error: InputsDirectoryError) -> Self {
        log::error!("{}\n", error);
        CLIError::InputsDirectoryError(error)
    }
}

impl From<InputsFileError> for CLIError {
    fn from(error: InputsFileError) -> Self {
        log::error!("{}\n", error);
        CLIError::InputsFileError(error)
    }
}

impl From<MainFileError> for CLIError {
    fn from(error: MainFileError) -> Self {
        log::error!("{}\n", error);
        CLIError::MainFileError(error)
    }
}

impl From<ManifestError> for CLIError {
    fn from(error: ManifestError) -> Self {
        log::error!("{}\n", error);
        CLIError::ManifestError(error)
    }
}

impl From<NewError> for CLIError {
    fn from(error: NewError) -> Self {
        log::error!("{}\n", error);
        CLIError::NewError(error)
    }
}

impl From<OutputsDirectoryError> for CLIError {
    fn from(error: OutputsDirectoryError) -> Self {
        log::error!("{}\n", error);
        CLIError::OutputsDirectoryError(error)
    }
}

impl From<ProofFileError> for CLIError {
    fn from(error: ProofFileError) -> Self {
        log::error!("{}\n", error);
        CLIError::ProofFileError(error)
    }
}

impl From<ProvingKeyFileError> for CLIError {
    fn from(error: ProvingKeyFileError) -> Self {
        log::error!("{}\n", error);
        CLIError::ProvingKeyFileError(error)
    }
}

impl From<RunError> for CLIError {
    fn from(error: RunError) -> Self {
        log::error!("{}\n", error);
        CLIError::RunError(error)
    }
}

impl From<SourceDirectoryError> for CLIError {
    fn from(error: SourceDirectoryError) -> Self {
        log::error!("{}\n", error);
        CLIError::SourceDirectoryError(error)
    }
}

impl From<TestError> for CLIError {
    fn from(error: TestError) -> Self {
        log::error!("{}\n", error);
        CLIError::TestError(error)
    }
}

impl From<VerificationKeyFileError> for CLIError {
    fn from(error: VerificationKeyFileError) -> Self {
        log::error!("{}\n", error);
        CLIError::VerificationKeyFileError(error)
    }
}

impl From<leo_compiler::errors::CompilerError> for CLIError {
    fn from(error: leo_compiler::errors::CompilerError) -> Self {
        log::error!("{}\n", error);
        CLIError::Crate("leo_compiler", "Program failed due to previous error".into())
    }
}

impl From<leo_inputs::errors::InputParserError> for CLIError {
    fn from(error: leo_inputs::errors::InputParserError) -> Self {
        log::error!("{}\n", error);
        CLIError::Crate("leo_inputs", "Program failed due to previous error".into())
    }
}

impl From<snarkos_errors::gadgets::SynthesisError> for CLIError {
    fn from(error: snarkos_errors::gadgets::SynthesisError) -> Self {
        CLIError::Crate("snarkos_errors", format!("{}", error))
    }
}

impl From<serde_json::error::Error> for CLIError {
    fn from(error: serde_json::error::Error) -> Self {
        CLIError::Crate("serde_json", format!("{}", error))
    }
}

impl From<std::io::Error> for CLIError {
    fn from(error: std::io::Error) -> Self {
        CLIError::Crate("std::io", format!("{}", error))
    }
}
