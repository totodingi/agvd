#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(about="African Genome Variation Database")]
#[command(propagate_version = true)]
struct CommandReader {
    #[command(subcommand)]
    command: Commands

}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Login User
    Login {
        /// User's unique ID
        #[arg(short,long)]
        user: String,
        /// User's Password
        #[arg(short,long)]
        password: String,
    },
    /// User Signup
    Signup {
        /// User id
        #[arg(short,long)]
        id: String,
        /// Full name
        #[arg(short,long)]
        name: String,
        /// Email
        #[arg(short,long)]
        email: String,
        /// Preferred password
        #[arg(short,long)]
        password: String,
        /// Affiliate organization
        #[arg(short,long)]
        organization: String,
    },
    /// Variant Query
    Query {
        /// User access token
        #[arg(short,long)]
        token: Option<String>,
        /// User ID
        #[arg(short,long)]
        user: Option<String>,
        /// User Password
        #[arg(short,long)]
        password: Option<String>,
        /// Variant ID
        #[arg(short,long)]
        id: Option<String>,
    }
}

#[derive(Args, Debug)]
struct Login {
    #[arg(short,long)]
    user: String,
    password: String,
}
