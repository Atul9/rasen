//! Mul trait implementation

use quote::{Ident, Tokens};
use codegen::defs::{Category, Node};
use codegen::math::construct_type;

fn impl_vector_times_scalar(result: Ident, size: u32, vector: Tokens, scalar: Tokens) -> Tokens {
    let v_fields: Vec<_> = {
        (0..size)
            .map(|i| Ident::from(format!("v_{}", i)))
            .collect()
    };
    let res_fields: Vec<_> = {
        v_fields.iter()
            .map(|f| {
                quote! { #f * other }
            })
            .collect()
    };

    quote! {
        let #result( #( #v_fields ),* ) = #vector;
        let other = #scalar;
        return #result( #( #res_fields ),* ).into();
    }
}

fn impl_vector_times_matrix(result: Ident, size: u32, vector: Tokens, matrix: Tokens) -> Tokens {
    let v_fields: Vec<_> = {
        (0..size)
            .map(|i| Ident::from(format!("v_{}", i)))
            .collect()
    };
    let res_fields: Vec<_> = {
        (0..size)
            .map(|i| {
                let sum: Vec<_> = {
                    (0..size)
                        .map(|j| {
                            let f = Ident::from(format!("v_{}", j));
                            let index = ((i * size) + j) as usize;
                            quote! { #f * matrix[#index] }
                        })
                        .collect()
                };

                quote! { #( #sum )+* }
            })
            .collect()
    };

    quote! {
        let #result( #( #v_fields ),* ) = #vector;
        let matrix = #matrix;
        return #result( #( #res_fields ),* ).into();
    }
}

#[cfg_attr(feature="clippy", allow(match_same_arms))]
pub fn impl_mul_variant(left_type: Node, right_type: Node) -> Option<Tokens> {
    let left_res = left_type.result.clone();
    let right_res = right_type.result.clone();

    let (result, mul_impl) = match (left_res.category, left_res.ty, right_res.category, right_res.ty) {
        (_, "bool", _, _) |
        (_, _, _, "bool") |
        (Category::SCALAR, _, Category::SCALAR, _) => return None,

        (lc, lt, rc, rt) if lc == rc && lt == rt && left_res.size == right_res.size => (
            left_res.name.clone(),
            match lc {
                Category::SCALAR => {
                    quote! {
                        return (left_val * right_val).into();
                    }
                },
                Category::VECTOR => {
                    let result = left_res.name.clone();
                    let l_fields: Vec<_> = {
                        (0..left_res.size.unwrap())
                            .map(|i| Ident::from(format!("l_{}", i)))
                            .collect()
                    };
                    let r_fields: Vec<_> = {
                        (0..left_res.size.unwrap())
                            .map(|i| Ident::from(format!("r_{}", i)))
                            .collect()
                    };
                    let res_fields: Vec<_> = {
                        l_fields.iter()
                            .zip(r_fields.iter())
                            .map(|(l_f, r_f)| {
                                quote! { #l_f * #r_f }
                            })
                            .collect()
                    };

                    quote! {
                        let #result( #( #l_fields ),* ) = left_val;
                        let #result( #( #r_fields ),* ) = right_val;
                        return #result( #( #res_fields ),* ).into();
                    }
                },
                Category::MATRIX => {
                    let result = left_res.name.clone();
                    let size = left_res.size.unwrap() as usize;
                    let res_fields: Vec<_> = {
                        (0..(size*size))
                            .map(|i| quote! { left_mat[#i] * right_mat[#i] })
                            .collect()
                    };

                    quote! {
                        let left_mat = left_val.0;
                        let right_mat = right_val.0;
                        return #result([ #( #res_fields ),* ]).into();
                    }
                },
            }
        ),

        (Category::VECTOR, lt, rc, rt) if lt == rt && left_res.size.unwrap() == right_res.size.or(left_res.size).unwrap() => (
            left_res.name.clone(),
            match rc {
                Category::VECTOR => unreachable!(),
                Category::SCALAR => {
                    impl_vector_times_scalar(
                        left_res.name.clone(),
                        left_res.size.unwrap(),
                        quote! { left_val },
                        quote! { right_val },
                    )
                },
                Category::MATRIX => impl_vector_times_matrix(
                    left_res.name.clone(),
                    left_res.size.unwrap(),
                    quote! { left_val },
                    quote! { right_val.0 },
                ),
            }
        ),
        (lc, lt, Category::VECTOR, rt) if lt == rt && right_res.size.unwrap() == left_res.size.or(right_res.size).unwrap() => (
            right_res.name.clone(),
            match lc {
                Category::VECTOR => unreachable!(),
                Category::SCALAR => {
                    impl_vector_times_scalar(
                        right_res.name.clone(),
                        right_res.size.unwrap(),
                        quote! { right_val },
                        quote! { left_val },
                    )
                },
                Category::MATRIX => impl_vector_times_matrix(
                    right_res.name.clone(),
                    right_res.size.unwrap(),
                    quote! { right_val },
                    quote! { left_val.0 },
                ),
            }
        ),

        _ => return None,
    };

    let left_type = construct_type(left_type);
    let right_type = construct_type(right_type);

    Some(quote! {
        impl Mul<#right_type> for #left_type {
            type Output = Value<#result>;

            #[inline]
            fn mul(self, rhs: #right_type) -> Self::Output {
                if let (Some(left_val), Some(right_val)) = (self.get_concrete(), rhs.get_concrete()) {
                    #mul_impl
                }

                let graph_opt = self.get_graph().or(rhs.get_graph());
                if let Some(graph_ref) = graph_opt {
                    let left_src = self.get_index(graph_ref.clone());
                    let right_src = rhs.get_index(graph_ref.clone());

                    let index = {
                        let mut graph = graph_ref.borrow_mut();
                        let index = graph.add_node(Node::Multiply);
                        graph.add_edge(left_src, index, 0);
                        graph.add_edge(right_src, index, 1);
                        index
                    };

                    return Value::Abstract {
                        graph: graph_ref.clone(),
                        index,
                        ty: PhantomData,
                    };
                }

                unreachable!()
            }
        }
    })
}
