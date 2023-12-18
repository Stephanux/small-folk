/*Ce module doit permettre de charger les routes de façon
dynamique depuis une fichier json de confguration */

pub mod dynamic_router {
    use tide::Server;
    use crate::appdyn::appdyn::State;

    pub fn routing(_app: Server<State>) {
    
    }

}
