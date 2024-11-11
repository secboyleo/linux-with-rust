use std::io::{self};

//=================================== ESTRUTURAS DE DADOS ===================================

#[derive(Debug, Clone)]
struct Arquivo {
    nome: String,
    tamanho: u16,
    permissao: (Permissao, Permissao, Permissao),
    usuario: u16,
    grupo: u16,
}

#[derive(Debug, Clone, Copy)]
struct Permissao {
    r: bool,
    w: bool,
    x: bool,
}

#[derive(Debug, Clone)]
struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    dono: String,
}

#[derive(Debug, Clone)]
struct Usuario {
    nome: String,
    uid: u16,
    grupo: Grupo,
    grupos: Vec<Grupo>,
}

#[derive(Debug, Clone)]
struct Grupo {
    nome: String,
    gid: u16,
    membros: Vec<Usuario>,
}

//============================ IMPLEMENTAÇÃO DAS FUNÇÕES ==============================

impl Arquivo {
    fn new(nome: String, tamanho: u16, permissao: (Permissao, Permissao, Permissao), usuario: u16, grupo: u16) -> Arquivo {
        Arquivo { nome, tamanho, permissao, usuario, grupo }
    }

    fn stat(&self) {
        println!("Nome: {}", self.nome);
        println!("Tamanho: {}", self.tamanho);
        let (permissao_usuario, permissao_grupo, permissao_outros) = self.permissao;
        println!("Permissao: {}{}{}", permissao_usuario.octal(), permissao_grupo.octal(), permissao_outros.octal());
        println!("Uid: {:?}", self.usuario);
        println!("Gid: {:?}", self.grupo);
    }

    fn alterar_permissao(&mut self, permissao: (Permissao, Permissao, Permissao)) {
        self.permissao = permissao;
    }
}

impl Permissao {
    fn new(r: bool, w: bool, x: bool) -> Permissao {
        Permissao { r, w, x }
    }

    fn octal(&self) -> u8 {
        let r: u8 = 4;
        let w: u8 = 2;
        let x: u8 = 1;
        let mut soma: u8 = 0;

        if self.r { soma += r; }
        if self.w { soma += w; }
        if self.x { soma += x; }

        soma
    }
}

impl Diretorio {
    fn new(nome: String, permissoes: (Permissao, Permissao, Permissao), dono: String) -> Diretorio {
        Diretorio { nome, arquivos: Vec::new(), permissoes, dono }
    }

    fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }

    fn remove_arquivo(&mut self, nome: String) {
        self.arquivos.retain(|x| x.nome != nome);
    }

    fn listar_conteudo(&self) {
        for arquivo in self.arquivos.iter() {
            println!("{}", arquivo.nome);
        }
    }
}

impl Usuario {
    fn new(nome: String, uid: u16, grupo: Grupo) -> Usuario {
        Usuario { nome, uid, grupo, grupos: Vec::new() }
    }

    fn adiciona_grupo(&mut self, grupo: Grupo) {
        self.grupos.push(grupo);
    }

    fn remove_grupo(&mut self, nome: String) {
        self.grupos.retain(|g| g.nome != nome);
    }

    fn listar_grupos(&self) {
        for grupo in self.grupos.iter() {
            println!("{}", grupo.nome);
        }
    }
}

impl Grupo {
    fn new(nome: String, gid: u16) -> Grupo {
        Grupo { nome, gid, membros: Vec::new() }
    }

    fn adiciona_membro(&mut self, usuario: Usuario) {
        self.membros.push(usuario);
    }

    fn remove_membro(&mut self, nome: String) {
        self.membros.retain(|u| u.nome != nome);
    }

    fn listar_usuarios(&self) {
        for membro in self.membros.iter() {
            println!("{}", membro.nome);
        }
    }
}

//============================ FUNÇÕES AUXILIARES ==============================

fn ler_entrada() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro na leitura.");
    input.trim().to_string()
}

fn ler_u16() -> u16 {
    loop {
        let entrada = ler_entrada();
        if let Ok(valor) = entrada.parse::<u16>() {
            return valor;
        } else {
            println!("Por favor, insira um número válido.");
        }
    }
}

fn ler_bool(mensagem: &str) -> bool {
    println!("{}", mensagem);
    let entrada = ler_entrada();
    entrada == "1"
}

//============================ FUNÇÕES PARA CRIAÇÃO DE PERMISSÕES ==============================

fn criar_permissao_individual() -> Permissao {
    let r = ler_bool("Leitura (1 para permitido, 0 para negado): \n");
    let w = ler_bool("Escrita (1 para permitido, 0 para negado): \n");
    let x = ler_bool("Execução (1 para permitido, 0 para negado): \n");
    Permissao::new(r, w, x)
}

fn criar_permissao() -> (Permissao, Permissao, Permissao) {
    println!("Digite permissões para o usuário (Leitura, Escrita, Execução):");
    let user = criar_permissao_individual();
    println!("Digite permissões para o grupo (Leitura, Escrita, Execução):");
    let group = criar_permissao_individual();
    println!("Digite permissões para outros (Leitura, Escrita, Execução):");
    let others = criar_permissao_individual();
    (user, group, others)
}

//============================ FUNÇÕES PARA OS MENUS ==============================

fn menu_grupos(grupos: &mut Vec<Grupo>, usuarios: &mut Vec<Usuario>) {
    loop {
        println!("\n==== Menu Grupos ====");
        println!("[1] Criar Grupo");
        println!("[2] Listar Grupos");
        println!("[3] Adicionar Membro a Grupo");
        println!("[4] Remover Membro de Grupo");
        println!("[0] Voltar");

        let escolha = ler_entrada();

        match escolha.as_str() {
            "1" => {
                println!("Digite o nome do grupo:");
                let nome = ler_entrada();
                println!("Digite o GID do grupo:");
                let gid = ler_u16();
                grupos.push(Grupo::new(nome, gid));
                println!("Grupo criado com sucesso!");
            }

            "2" => {
                if grupos.is_empty() {
                    println!("Não há grupos para listar.");
                } else {
                    println!("Lista de Grupos:");
                    for grupo in grupos.iter() {
                        println!("Grupo: {}, GID: {}", grupo.nome, grupo.gid);
                        grupo.listar_usuarios();
                    }
                }
            }

            "3" => {
                println!("Digite o nome do grupo para adicionar um membro:");
                let nome_grupo = ler_entrada();
                if let Some(grupo) = grupos.iter_mut().find(|g| g.nome == nome_grupo) {
                    println!("Digite o nome do usuário para adicionar ao grupo:");
                    let nome_usuario = ler_entrada();
                    if let Some(usuario) = usuarios.iter().find(|u| u.nome == nome_usuario) {
                        grupo.adiciona_membro(usuario.clone());
                        println!("Usuário adicionado ao grupo com sucesso!");
                    } else {
                        println!("Usuário não encontrado.");
                    }
                } else {
                    println!("Grupo não encontrado.");
                }
            }

            "4" => {
                println!("Digite o nome do grupo para remover um membro:");
                let nome_grupo = ler_entrada();
                if let Some(grupo) = grupos.iter_mut().find(|g| g.nome == nome_grupo) {
                    println!("Digite o nome do usuário para remover do grupo:");
                    let nome_usuario = ler_entrada();
                    grupo.remove_membro(nome_usuario);
                    println!("Usuário removido do grupo com sucesso!");
                } else {
                    println!("Grupo não encontrado.");
                }
            }

            "0" => break,

            _ => println!("Opção inválida, tente novamente."),
        }
    }
}

fn menu_usuarios(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
    loop {
        println!("\n==== Menu Usuários ====");
        println!("[1] Criar Usuário");
        println!("[2] Listar Usuários");
        println!("[3] Adicionar Grupo ao Usuário");
        println!("[4] Remover Grupo do Usuário");
        println!("[0] Voltar");

        let escolha = ler_entrada();

        match escolha.as_str() {
            "1" => {
                println!("Digite o nome do usuário:");
                let nome = ler_entrada();
                println!("Digite o UID do usuário:");
                let uid = ler_u16();
                println!("Digite o nome do grupo principal do usuário:");
                let nome_grupo = ler_entrada();
                if let Some(grupo) = grupos.iter().find(|g| g.nome == nome_grupo) {
                    usuarios.push(Usuario::new(nome, uid, grupo.clone()));
                    println!("Usuário criado com sucesso!");
                } else {
                    println!("Grupo principal não encontrado.");
                }
            }

            "2" => {
                if usuarios.is_empty() {
                    println!("Não há usuários para listar.");
                } else {
                    println!("Lista de Usuários:");
                    for usuario in usuarios.iter() {
                        println!("Usuário: {}\n UID: {}", usuario.nome, usuario.uid);
                        usuario.listar_grupos();
                    }
                }
            }

            "3" => {
                println!("Digite o nome do usuário para adicionar a um grupo:");
                let nome_usuario = ler_entrada();
                if let Some(usuario) = usuarios.iter_mut().find(|u| u.nome == nome_usuario) {
                    println!("Digite o nome do grupo para adicionar:");
                    let nome_grupo = ler_entrada();
                    if let Some(grupo) = grupos.iter().find(|g| g.nome == nome_grupo) {
                        usuario.adiciona_grupo(grupo.clone());
                        println!("Grupo adicionado ao usuário com sucesso!");
                    } else {
                        println!("Grupo não encontrado.");
                    }
                } else {
                    println!("Usuário não encontrado.");
                }
            }

            "4" => {
                println!("Digite o nome do usuário para remover de um grupo:");
                let nome_usuario = ler_entrada();
                if let Some(usuario) = usuarios.iter_mut().find(|u| u.nome == nome_usuario) {
                    println!("Digite o nome do grupo para remover:");
                    let nome_grupo = ler_entrada();
                    usuario.remove_grupo(nome_grupo);
                    println!("Grupo removido do usuário com sucesso!");
                } else {
                    println!("Usuário não encontrado.");
                }
            }

            "0" => break,

            _ => println!("Opção inválida, tente novamente."),
        }
    }
}

fn menu_arquivos(arquivos: &mut Vec<Arquivo>) {
    loop {
        println!("\n==== Menu Arquivos ====");
        println!("[1] Criar Arquivo");
        println!("[2] Listar Arquivos");
        println!("[3] Alterar Permissões de Arquivo");
        println!("[0] Voltar");

        let escolha = ler_entrada();

        match escolha.as_str() {
            "1" => {
                println!("Digite o nome do arquivo:");
                let nome = ler_entrada();
                println!("Digite o tamanho do arquivo:");
                let tamanho = ler_u16();
                println!("Digite as permissões do arquivo:");
                let permissao = criar_permissao();
                println!("Digite o UID do arquivo:");
                let usuario = ler_u16();
                println!("Digite o GID do arquivo:");
                let grupo = ler_u16();
                arquivos.push(Arquivo::new(nome, tamanho, permissao, usuario, grupo));
                println!("Arquivo criado com sucesso!");
            }

            "2" => {
                if arquivos.is_empty() {
                    println!("Não há arquivos para listar.");
                } else {
                    println!("Lista de Arquivos:");
                    for arquivo in arquivos.iter() {
                        println!("{}", arquivo.nome);
                        arquivo.stat();
                    }
                }
            }

            "3" => {
                println!("Digite o nome do arquivo para alterar permissões:");
                let nome_arquivo = ler_entrada();
                if let Some(arquivo) = arquivos.iter_mut().find(|a| a.nome == nome_arquivo) {
                    println!("Digite as novas permissões do arquivo:");
                    let permissao = criar_permissao();
                    arquivo.alterar_permissao(permissao);
                    println!("Permissões do arquivo alteradas com sucesso!");
                } else {
                    println!("Arquivo não encontrado.");
                }
            }

            "0" => break,

            _ => println!("Opção inválida, tente novamente."),
        }
    }
}

fn menu_diretorios(diretorios: &mut Vec<Diretorio>, arquivos: &Vec<Arquivo>) {
    loop {
        println!("\n==== Menu Diretórios ====");
        println!("[1] Criar Diretório");
        println!("[2] Listar Conteúdo de Diretório");
        println!("[3] Adicionar Arquivo a Diretório");
        println!("[4] Remover Arquivo de Diretório");
        println!("[0] Voltar");

        let escolha = ler_entrada();

        match escolha.as_str() {
            "1" => {
                println!("Digite o nome do diretório:");
                let nome = ler_entrada();
                println!("Digite as permissões do diretório:");
                let permissoes = criar_permissao();
                println!("Digite o nome do dono do diretório:");
                let dono = ler_entrada();
                diretorios.push(Diretorio::new(nome, permissoes, dono));
                println!("Diretório criado com sucesso!");
            }

            "2" => {
                println!("Digite o nome do diretório para listar o conteúdo:");
                let nome_diretorio = ler_entrada();
                if let Some(diretorio) = diretorios.iter().find(|d| d.nome == nome_diretorio) {
                    println!("Conteúdo do diretório '{}':", nome_diretorio);
                    diretorio.listar_conteudo();
                } else {
                    println!("Diretório não encontrado.");
                }
            }

            "3" => {
                println!("Digite o nome do diretório para adicionar um arquivo:");
                let nome_diretorio = ler_entrada();
                if let Some(diretorio) = diretorios.iter_mut().find(|d| d.nome == nome_diretorio) {
                    println!("Digite o nome do arquivo para adicionar:");
                    let nome_arquivo = ler_entrada();
                    if let Some(arquivo) = arquivos.iter().find(|a| a.nome == nome_arquivo) {
                        diretorio.adiciona_arquivo(arquivo.clone());
                        println!("Arquivo adicionado ao diretório com sucesso!");
                    } else {
                        println!("Arquivo não encontrado.");
                    }
                } else {
                    println!("Diretório não encontrado.");
                }
            }

            "4" => {
                println!("Digite o nome do diretório para remover um arquivo:");
                let nome_diretorio = ler_entrada();
                if let Some(diretorio) = diretorios.iter_mut().find(|d| d.nome == nome_diretorio) {
                    println!("Digite o nome do arquivo para remover:");
                    let nome_arquivo = ler_entrada();
                    diretorio.remove_arquivo(nome_arquivo);
                    println!("Arquivo removido do diretório com sucesso!");
                } else {
                    println!("Diretório não encontrado.");
                }
            }

            "0" => break,

            _ => println!("Opção inválida, tente novamente."),
        }
    }
}


//============================ FUNÇÃO PRINCIPAL ==============================

fn main() {
    let mut arquivos = Vec::new();
    let mut diretorios = Vec::new();
    let mut usuarios = Vec::new();
    let mut grupos = Vec::new();

    loop {
        println!("\nBEM VINDO AO XUNIL");
        println!("[1] Arquivos");
        println!("[2] Diretorios");
        println!("[3] Usuários");
        println!("[4] Grupos");
        println!("[0] Sair");

        let escolha = ler_entrada();

        match escolha.as_str() {
            "1" => menu_arquivos(&mut arquivos),
            "2" => menu_diretorios(&mut diretorios, &arquivos),
            "3" => menu_usuarios(&mut usuarios, &mut grupos),
            "4" => menu_grupos(&mut grupos, &mut usuarios),
            "0" => break,
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}
