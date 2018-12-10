use super::rpc::*;
use super::common::*;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Client{
    id: Uuid,  
    seq: i32,
    pub leader: usize,
    pub servers: Vec<String>,
}

impl Client{

    pub fn new(servers : Vec<String>) -> Client {
        Client {
            id: Uuid::new_v4(),
            seq: 1,
            leader: 0,
            servers: servers,
        }
    }

    fn getseq(&mut self) -> i32{
        let x: i32 = self.seq;
        self.seq +=1;
        x
    }

    pub fn get(&mut self,key:String) -> String {
        let getargs = GetArgs {
            key:key,
            id:self.id,
            seq:self.getseq(),
        };
        let getargs = serde_json::to_string(&getargs).unwrap();

        let mut i  = self.leader;
        loop {
            let (ok,reply) = rpc_call(self.servers[i].to_string(),"Kv.Get".to_string(),getargs.clone());
            if ok == false || reply.ok == false {
                i = (i+1) % 5;
                continue;
            }
            let getreply: GetReply=serde_json::from_str(&reply.reply).unwrap();
            if  getreply.wrong_leader == false {
                self.leader = i;
                if getreply.err == "OK" {
                    return getreply.value ;
                } else {
                    println!("Not find the value of key");
                    return String::from("");
                }
            } else {
                i = (i+1)% 5;
            }
        }
    }

    fn put_append(&mut self, key: String,value: String,op: String){
        let putargs = PutAppendArgs {
            key: key.clone(),
            value: value.clone(),
            op: op,
            id: self.id,
            seq: self.getseq(),
        };
        let putargs = serde_json::to_string(&putargs).unwrap();

        let mut i =self.leader;
        loop {
            //println!("Try to call leader: {}", self.servers[i]);
            let (ok,reply) = rpc_call(self.servers[i].to_string(),"Kv.Put".to_string(),putargs.clone());
           // println!("{}",i);
           // println!("Reply is: {:?} ", reply);
            if ok == false || reply.ok == false {
                i = (i+1) % 5;
                continue;
            }
            let putreply: PutAppendReply = serde_json::from_str(&reply.reply).unwrap();
            if putreply.wrong_leader == false {
                println!("put {} : {} ok",key,value);
                self.leader=i;
                return ;
            }
            else {
                i=(i+1)%5;
            }
        }
    }

    pub fn put(&mut self, key: String,value:String) {
        self.put_append(key,value,String::from("Put"));
    }

    pub fn append(&mut self, key: String,value:String) {
        self.put_append(key,value,String::from("PutAppend"));
    }
}
