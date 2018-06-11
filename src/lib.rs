#![feature(test)]
extern crate test;
use test::*;



pub fn for_every_pair_iter<T>(mut arr:&mut [T],mut func:impl FnMut(&mut T,&mut T)){
    loop{
    	let temp=arr;
    	match temp.split_first_mut(){
    		Some((b1,x))=>{
    			for b2 in x.iter_mut(){
    				func(b1,b2);
    			}
    			arr=x;
    		},
    		None=>break
    	}
    }
}

pub fn for_every_pair_recc<T>(rest:&mut [T],mut func: impl FnMut(&mut T,&mut T)){
    match rest.split_first_mut(){
        Some((b1,rest))=>{
            for b2 in rest.iter_mut(){
                func(b1,b2);
            } 
            for_every_pair_recc(rest,func);
        },
        None=>{}
    }
}

pub fn for_every_pair_unsafe_impl<T>(arr:&mut [T],mut func:impl FnMut(&mut T,&mut T)){
    unsafe{
        for x in 0..arr.len(){
            let xx=arr.get_unchecked_mut(x) as *mut T;
            for j in (x+1)..arr.len(){
                let j=arr.get_unchecked_mut(j);
                let xx=&mut*xx;
                func(xx,j);
            }
        }
    }
}



macro_rules! gen_test{
    ($test:ident,$bench:ident,$function:ident)=>{
    	#[test]
    	fn $test(){
    		let mut k=[0;100];
			$function(&mut k,|a,b|{
				*a+=1;
				*b+=1;
			});

			for b in k.iter(){
				assert!(*b==100-1);
			}
    	}

    	#[bench]
    	fn $bench(bench:&mut Bencher){
    		let mut k=[0;2000];
			bench.iter(||{
				$function(&mut k,|a,b|{
					*a+=1;
					*b+=1;
				})
			});


			black_box(k);
    	}
    }
}
gen_test!(iter_test,iter_bench,for_every_pair_iter);
gen_test!(recc_test,recc_bench,for_every_pair_recc);
gen_test!(unsafe_test,unsafe_bench,for_every_pair_unsafe_impl);