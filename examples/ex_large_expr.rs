use std::collections::HashMap;
use std::rc::Rc;
use symoxide as sym;
use symoxide::mappers::identity::IdentityMapper;
use symoxide::mappers::CachedMapper;

#[derive(sym::CachedMapper)]
struct Renamer {
    cache: HashMap<sym::ExpressionRawPointer, Rc<sym::Expression>>,
}

impl sym::mappers::identity::IdentityMapper for Renamer {
    fn map_variable(&mut self, name: String) -> Rc<sym::Expression> {
        let new_name = match name.as_str() {
            "iface_ensm15" => "_0",
            "iel_ensm15" => "_1",
            "idof_ensm15" => "_2",
            x => x,
        };
        sym::var(new_name)
    }
}

fn main() {
    // substitute iface_ensm15 => _0
    // substitute iel_ensm15   => _1
    // substitute idof_ensm15  => _2
    let code = concat!("(-1)*((cse_577[_pt_data_48[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0],",
                       "_pt_data_49[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_48[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_577[_pt_data_46[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0],",
                       " _pt_data_47[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_46[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_577[_pt_data_7[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0],",
                       " _pt_data_43[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_7[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_577[_pt_data_44[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0],",
                       " _pt_data_45[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_44[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_579[_pt_data_68[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0],",
                       " _pt_data_69[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_68[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_579[_pt_data_66[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0],",
                       " _pt_data_67[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_66[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_579[_pt_data_50[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0],",
                       " _pt_data_63[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_50[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_579[_pt_data_64[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0],",
                       " _pt_data_65[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_64[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_581[_pt_data_88[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0],",
                       " _pt_data_89[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_88[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_581[_pt_data_86[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0],",
                       " _pt_data_87[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_86[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_581[_pt_data_70[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_83[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_70[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_581[_pt_data_84[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_85[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_84[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_582[_pt_data_107[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0],",
                       " _pt_data_108[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_107[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_582[_pt_data_105[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0],",
                       " _pt_data_106[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_105[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_582[_pt_data_90[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_102[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_90[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_582[_pt_data_103[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_104[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_103[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0))",
                       " + (cse_572[_pt_data_48[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_49[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_48[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0) ",
                       "+ (cse_572[_pt_data_46[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_47[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_46[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0) ",
                       "+ (cse_572[_pt_data_7[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_43[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_7[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_572[_pt_data_44[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_45[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_44[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_573[_pt_data_68[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_69[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_68[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_573[_pt_data_66[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_67[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_66[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_573[_pt_data_50[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_63[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_50[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_573[_pt_data_64[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_65[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_64[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_574[_pt_data_88[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_89[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_88[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_574[_pt_data_86[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_87[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_86[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0) ",
                       "+ (cse_574[_pt_data_70[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_83[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_70[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_574[_pt_data_84[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_85[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_84[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_575[_pt_data_107[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_108[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_107[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_575[_pt_data_105[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_106[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_105[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_575[_pt_data_90[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_102[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_90[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)",
                       " + (cse_575[_pt_data_103[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0], _pt_data_104[(iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 10]]",
                       " if _pt_data_103[((iface_ensm15*1075540 + iel_ensm15*10 + idof_ensm15) % 4302160) // 10, 0] != -1 else 0)");

    let expr = sym::parse(code);
    let expr = sym::deduplicate_nodes(&expr);

    let t_start = std::time::Instant::now();
    for _ in 0..10_000 {
        let mut renamer = Renamer { cache: HashMap::new() };
        let _new_expr = renamer.visit(expr.clone());
    }
    println!("Took: {:?} secs", t_start.elapsed());
}
