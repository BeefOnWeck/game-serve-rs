
#[derive(Debug, PartialEq)]
pub enum Resource {
    Block,
    Rock,
    Timber,
    Fiber,
    Cereal,
    Desert
}

#[derive(Debug, PartialEq)]
pub struct ResourceList {
    block: u16,
    rock: u16,
    timber: u16,
    fiber: u16,
    cereal: u16
}

type ResourceArray<const N: usize> = [Resource; N];

impl ResourceList {
    pub fn new() -> ResourceList {
        ResourceList { block: 0, rock: 0, timber: 0, fiber: 0, cereal: 0 }
    }

    pub fn deposit(&mut self, resource: Resource) -> Result<(),&'static str> {
        match resource {
            Resource::Block => self.block += 1,
            Resource::Rock => self.rock += 1,
            Resource::Timber => self.timber += 1,
            Resource::Fiber => self.fiber += 1,
            Resource::Cereal => self.cereal += 1,
            Resource::Desert => return Err("Can't deposit Desert resources.")
        }

        Ok(())
    }

    pub fn deduct<const N: usize>(&mut self, resources: ResourceArray<N>) -> Result<(),&'static str> {
        for resource in resources {
            match resource {
                Resource::Block => 
                    if self.block > 0 { self.block -= 1 } 
                    else { return Err("Can't deduct; not enough resources.") },
                Resource::Rock => 
                    if self.rock > 0 { self.rock -= 1 } 
                    else { return Err("Can't deduct; not enough resources.") },
                Resource::Timber => 
                    if self.timber > 0 { self.timber -= 1 } 
                    else { return Err("Can't deduct; not enough resources.") },
                Resource::Fiber => 
                    if self.fiber > 0 { self.fiber -= 1 } 
                    else { return Err("Can't deduct; not enough resources.") },
                Resource::Cereal => 
                    if self.cereal > 0 { self.cereal -= 1 } 
                    else { return Err("Can't deduct; not enough resources.") },
                Resource::Desert => 
                    return Err("Can't deduct Desert resources.")
            }
        }

        Ok(())
    }
}

#[cfg(test)]

#[test]
fn resource_lists() {
    let mut resource_list = ResourceList::new();
    assert_eq!(
        (resource_list.block, resource_list.rock, resource_list.timber, resource_list.fiber, resource_list. cereal),
        (0, 0, 0, 0, 0)
    );

    let _status = resource_list.deposit(Resource::Block);
    assert_eq!(
        (resource_list.block, resource_list.rock, resource_list.timber, resource_list.fiber, resource_list. cereal),
        (1, 0, 0, 0, 0)
    );

    let _status = resource_list.deposit(Resource::Rock);
    let _status = resource_list.deposit(Resource::Timber);
    let _status = resource_list.deposit(Resource::Fiber);
    let _status = resource_list.deposit(Resource::Cereal);
    assert_eq!(
        (resource_list.block, resource_list.rock, resource_list.timber, resource_list.fiber, resource_list. cereal),
        (1, 1, 1, 1, 1)
    );

    let _status = resource_list.deduct([Resource::Block, Resource::Rock, Resource::Timber]);
    assert_eq!(
        (resource_list.block, resource_list.rock, resource_list.timber, resource_list.fiber, resource_list. cereal),
        (0, 0, 0, 1, 1)
    );

}
