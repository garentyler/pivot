use pivot::ast::AstNode;
use regex::Regex;

fn main() {
    pivot::parse::parse();
    // let src = AstNode::program(vec![
    //     AstNode::add(
    //         AstNode::integer(2),
    //         AstNode::integer(3),
    //     )
    // ]);
    // println!("{:?}", src);
    // let mut out = String::new();
    // src.emit(&mut out);
    // println!("{}", out);
    // test();
}

// fn test() {
//     use pivot::ast::AstNode;
//
//     let src = r#"
//     function factorial(n) {
//         var result = 1;
//         while (n != 1) {
//             result = result * n;
//             n = n - 1;
//         }
//         return result;
//     }
//     "#;
//
//     let actual_ast = pivot::parse::parse();
//     let expected_ast = AstNode::function("factorial", vec!["n"],
//         AstNode::block(vec![
//             AstNode::variable("result", AstNode::number(1)),
//             AstNode::r#while(
//                 AstNode::not_equal(
//                     AstNode::identifier("n"),
//                     AstNode::number(1)
//                 ),
//                 AstNode::block(vec![
//                     AstNode::assign("result",
//                         AstNode::multiply(
//                             AstNode::identifier("result"),
//                             AstNode::identifier("n")
//                         )
//                     ),
//                     AstNode::assign("n",
//                         AstNode::subtract(
//                             AstNode::identifier("n"),
//                             AstNode::number(1)
//                         )
//                     )
//                 ]),
//             ),
//             AstNode::r#return(AstNode::identifier("result"))
//         ])
//     );
//     println!("{}", expected_ast);
// }
