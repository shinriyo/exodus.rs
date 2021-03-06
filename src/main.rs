extern crate curl;
use std::fs;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::collections::HashMap;
use std::mem;
use std::fs::metadata;
use std::path::Path;
use curl::http;

// 普通のstringをstaticに変換
fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}

fn main() {
    // exodus g item name:string price:integer description:text
    // カレントパスにsrcフォルダやCargo.tomlあるか？
    let path = Path::new("src");
    if !path.exists() {
        println!("No src folder Error.");
        return;
    }

    let path = Path::new("src/main.rs");
    if !path.exists() {
        println!("No src/main.rs file Error.");
        return;
    }

    let path = Path::new("Cargo.toml");
    if !path.exists() {
        println!("No Cargo.toml Error.");
        return;
    }

    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        println!("Parameter not enough Error.");
        // コンパイルの時もここを通るはず
        return;
    }

    /*
    パラメータ実験
    let x = "release_year:integer".to_string();
    let d: Vec<_> = x.split(':').collect();
    if d.len() != 2 {
        println!("format");
    }
    println!("{}", d[0]);
    println!("{}", d[1]);
    */

    // 後で変える名前
    let mut name = "item".to_string();
    let mut command_name = "g".to_string();

    let mut column_stack = Vec::new();
    let mut type_stack = Vec::new();

    let mut arg_idx = 0;
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    // indexのため
    let mut first_column: String = String::from("!");

    for argument in args {
        if arg_idx == 0 {
            // これはこのスクリプトの名前
        }
        else if arg_idx == 1 {
            // コマンド名
            // "g" or "generate"
            command_name = argument.to_string();
            if command_name == "g" || command_name == "generate" {
                if args_len <= 3 {
                    println!("[Err] Argument aren't enough.");
                    println!("Remember the arguments, to keep more than 3 argments.");
                    return;
                }

                println!("Generate Scaffolding....");
            } else if command_name == "init" || command_name == "initialie" {
                println!("Initialize....");
                let mut main_f = File::create("src/main.rs").unwrap();
                let main_raw = format!(r#"#[macro_use] extern crate nickel;
extern crate postgres;
extern crate openssl;
extern crate hyper;
use nickel::{{Nickel, StaticFilesHandler}};
use postgres::{{Connection, SslMode}};
use std::sync::{{Arc, Mutex}};

extern crate rustc_serialize;

// later you add module there!

fn main() {{
    let mut server = Nickel::new();
    server.utilize(StaticFilesHandler::new("app/assets/"));

    let mut router = Nickel::router();
    let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
    let shared_connection = Arc::new(Mutex::new(conn));
    // later you add scaffolding url there!

    server.utilize(router);
    server.listen("localhost:6767");
}}
"#);
                main_f.write_all(main_raw.as_bytes());
                println!("Generated template in src/main.rs.");

                // 初期化処理
                println!("Generated AngularJS libraries.");

                // フォルダ生成
                let path = "app/assets/css";
                match fs::create_dir_all(path) {
                    Err(why) => println!("! {:?}", why.kind()),
                    Ok(_) => {},
                }

                let path = "app/assets/lib";
                match fs::create_dir_all(path) {
                    Err(why) => println!("! {:?}", why.kind()),
                    Ok(_) => {},
                }

                let resp = http::handle()
                .get("https://ajax.googleapis.com/ajax/libs/angularjs/1.5.0/angular.min.js")
                .exec().unwrap();
                let str_body = String::from_utf8(resp.get_body().to_vec()).unwrap();
                let mut js_f = File::create("app/assets/lib/angular.min.js").unwrap();
                js_f.write_all(str_body.as_bytes());

                // route
                let resp = http::handle()
//                .get("https://ajax.googleapis.com/ajax/libs/angularjs/1.3.0/angular-route.min.js")
                .get("https://cdn.rawgit.com/angular-ui/ui-router/0.2.18/release/angular-ui-router.min.js")
                .exec().unwrap();
                let str_body = String::from_utf8(resp.get_body().to_vec()).unwrap();
                let mut js_f = File::create("app/assets/lib/angular-ui-router.min.js").unwrap();
                js_f.write_all(str_body.as_bytes());

                // resource
                let resp = http::handle()
                .get("https://ajax.googleapis.com/ajax/libs/angularjs/1.3.0/angular-resource.min.js")
                .exec().unwrap();
                let str_body = String::from_utf8(resp.get_body().to_vec()).unwrap();
                let mut js_f = File::create("app/assets/lib/angular-resource.min.js").unwrap();
                js_f.write_all(str_body.as_bytes());

                // CSS
                let mut css_f = File::create("app/assets/css/app.css").unwrap();
                let css_raw = r#".top-buffer{
    margin-top:10px;
}

.movietable tr td:nth-child(2){
    width: 150px;
}

.movietable tr:nth-child(1) td{
    border-top: none;
}
a.nodecoration{
    text-decoration:none;
}
"#;
                css_f.write_all(css_raw.as_bytes());

                let resp = http::handle()
                .get("http://maxcdn.bootstrapcdn.com/bootstrap/3.1.1/css/bootstrap.min.css")
                .exec().unwrap();
                let str_body = String::from_utf8(resp.get_body().to_vec()).unwrap();

                let mut css_f = File::create("app/assets/css/bootstrap.min.css").unwrap();
                css_f.write_all(str_body.as_bytes());

                println!("Finish initialize.");
                println!("You shall take [dependencies] of the Cargo.toml your project.");
                println!(r#"
[dependencies]
nickel = "*"
postgres = "0.11"
openssl = "*"
rustc-serialize = "*"
hyper = "*"
                "#);

                return;
            } else if command_name == "migrate" {
                println!("Migrate DB.");

                return;
            } else {
                println!("{} command not found.", command_name);
                return;
            }
        }
        else if arg_idx == 2 {
            // モデル名
            name = argument.to_string();
//            println!("model name: {}", name);
        } else {
            let d:Vec<_> = argument.split(':').collect();
            if arg_idx == 3 {
                first_column = d[0].to_string();
            }

//            println!("{}", d[0]);
            column_stack.push(d[0].to_string());
            type_stack.push(d[1].to_string());
        }

        arg_idx+=1;
    }

    // 大文字最初の名前
    let capitalized_name = format!("{}{}", &name[0..1].to_uppercase(), &name[1..name.len()]);

    // partials/_form.html用
    let mut form_html_as_str: Vec<String> = Vec::new();

    // partials/hoge-view.html用
    let mut view_html_as_str: Vec<String> = Vec::new();

    // CREATE TABLE
    let mut create_table_as_str: Vec<String> = Vec::new();
    // $1, $2, $3, $4
    let mut create_table_val_as_str: Vec<String> = Vec::new();

    // SELECT
    let mut select_table_str: Vec<String> = Vec::new();

    // UPDATE
    let mut update_sql_as_str: Vec<String> = Vec::new();

    // params
    let mut params_sql_as_str: Vec<String> = Vec::new();

    // struct
    let mut struct_as_str: Vec<String> = Vec::new();

    // json to object
    let mut json_to_obj_as_str: Vec<String> = Vec::new();

    let mut idx = 0;

    for column in &column_stack {
        let capitalized_column = format!("{}{}", &column[0..1].to_uppercase(), &column[1..column.len()]);
        // _form
        let raw = format!(r#"
<div class="form-group">
    <label for="{1}" class="col-sm-2 control-label">{2}</label>
    <div class="col-sm-10">
        <input type="text" ng-model="{0}.{1}" class="form-control" id="{1}" placeholder="{0}'s {2}"/>
    </div>
</div>
"#, name, column, capitalized_column);
        form_html_as_str.push(raw);

        // hoge-view
        let raw = format!(r#"
    <tr>
        <td>{1} {3}</td>
        <td>{{{{{0}.{2}}}}}</td>
    </tr>"#, name, capitalized_name, column, capitalized_column);
        view_html_as_str.push(raw);

        let mut comma = ", ";
        if (column_stack.len() - 1) == idx {
            comma = "";
        }

        // CREATE TABLE
        // scaffolding → Postgres Type
        let db_column_type;
        // scaffolding → Rust Type
        let rest_type;
        // json to obj用サポート
        let support;

        let scaffoding_type = type_stack.pop().unwrap();

        match (string_to_static_str(scaffoding_type)) {
            "bool" => {
                db_column_type = "BOOL";
                rest_type = "bool";
                support = "";
            }
            "integer" => {
                db_column_type = "SMALLINT";
                rest_type = "i16";
                support = "";
            }
            "string" => {
                db_column_type = "VARCHAR(50)";
                rest_type = "String";
                support = ".to_string()";
            }
            _ => {
                db_column_type = "VARCHAR(50)";
                rest_type = "String";
                support = ".to_string()";
            }
        }

        let raw = format!("{0} {1} NOT NULL{2}",
        column, db_column_type, comma);
        create_table_as_str.push(raw);

        let raw = format!("${0}{1}", idx+1, comma);
        create_table_val_as_str.push(raw);

        // SELECT
        let raw = format!("{0}{1}", column, comma);
        select_table_str.push(raw);

        // UPDATE
        let raw = format!("{0}=${1}{2}", column, idx+1, comma);
        update_sql_as_str.push(raw);

        // sql用のparam
        let raw = format!("{0}: row.get({1}){2}", column, idx+1, comma);
        params_sql_as_str.push(raw);

        // struct
        let raw = format!("{0}: {1}{2}", column, rest_type, comma);
        struct_as_str.push(raw);

        // json to obj
        // TODO:
        let raw = format!("&{0}.{1}{2}{3}", name, column, support, comma);
        json_to_obj_as_str.push(raw);

        idx += 1;
    }

    // CREATE TABLE
    let create_table_sql = format!("CREATE TABLE {0} (id SERIAL PRIMARY KEY, {1})",
        name, create_table_as_str.iter().cloned().collect::<String>());

    // SELECT ALL
    let select_sql = format!("SELECT id, {0} FROM {1}", select_table_str.iter().cloned().collect::<String>(), name);

    // INSERT
    let insert_sql = format!("INSERT INTO {1} ({0}) VALUES ({2})", select_table_str.iter().cloned().collect::<String>(),
        name,
        create_table_val_as_str.iter().cloned().collect::<String>());

    // UPDATE
    let update_sql = format!("UPDATE {1} SET {0} WHERE id = ${2}", update_sql_as_str.iter().cloned().collect::<String>(),
        name, column_stack.len() + 1);

    // SQLのparams
    let sql_params = format!("_id: row.get(0), {}", params_sql_as_str.iter().cloned().collect::<String>());

    // struct
    let struct_params = format!("{}", struct_as_str.iter().cloned().collect::<String>());

    // json to object用
    let json_to_obj = format!("{}", json_to_obj_as_str.iter().cloned().collect::<String>());

    // 開始
    // フォルダ生成
    match fs::create_dir_all(format!("app/assets/{}/partials", name)) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    /*
        HTML系のファイルたち
    */
    // ファイル
    // item/partials/_form.html
    let mut html_f = File::create(format!("app/assets/{0}/partials/_form.html", name)).unwrap();
    let html_raw = format!(r#"{}
<div class="form-group">
    <div class="col-sm-offset-2 col-sm-10">
        <input type="submit" class="btn btn-primary" value="Save"/>
    </div>
</div>
"#, form_html_as_str.iter().cloned().collect::<String>());
    html_f.write_all(html_raw.as_bytes());

    // item/partials/hoge-view.html
    let mut html_f = File::create(format!("app/assets/{0}/partials/{0}-view.html", name)).unwrap();
    let html_raw = format!(r#"<table class="table {0}table">
    <tr>
        <td><h3>Details for {{{{{0}.{3}}}}}</h3></td>
        <td></td>
    </tr>
    {2}
</table>
<div>
    <a class="btn btn-primary" ui-sref="edit{1}({{id:{0}._id}})">Edit</a>
</div>
"#, name, capitalized_name, view_html_as_str.iter().cloned().collect::<String>(), first_column);
    html_f.write_all(html_raw.as_bytes());

    // item/partials/hoge-add.html
    let mut add_f = File::create(format!("app/assets/{0}/partials/{0}-add.html", name)).unwrap();
    let html_raw = format!(r#"<form class="form-horizontal" role="form" ng-submit="add{1}()">
    <div ng-include="'{}/partials/_form.html'"></div>
</form>"#, name, capitalized_name);
    add_f.write_all(html_raw.as_bytes());

    // item/partials/hoge-edit.html
    let mut html_f = File::create(format!("app/assets/{0}/partials/{0}-edit.html", name)).unwrap();
    let html_raw = format!(r#"<form class="form-horizontal" role="form" ng-submit="update{1}()">
    <div ng-include="'{0}/partials/_form.html'"></div>
</form>"#, name, capitalized_name);
    html_f.write_all(html_raw.as_bytes());

    // 複数形
    // まだ仮実装
    let mut html_f = File::create(format!("app/assets/{0}/partials/{0}s.html", name)).unwrap();
    let html_raw = format!(r#"<a ui-sref="new{1}" class="btn-primary btn-lg nodecoration">Add New {1}</a>
<table class="table {0}table">
    <tr>
        <td><h3>All {1}s</h3></td>
        <td></td>
    </tr>
    <tr ng-repeat="{0} in {0}s">
        <td>{{{{{0}.{2}}}}}</td>
        <td>
            <a class="btn btn-primary" ui-sref="view{1}({{id:{0}._id}})">View</a>
            <a class="btn btn-danger"  ng-click="delete{1}({0})">Delete</a>
        </td>
    </tr>
</table>
"#, name, capitalized_name, first_column);
    html_f.write_all(html_raw.as_bytes());

    /*
        js系
    */
    match fs::create_dir_all(format!("app/assets/{}/js", name)) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    // ファイル
    let mut js_app_f = File::create(format!("app/assets/{}/js/filters.js", name)).unwrap();
    let js_app_raw = format!(r#"/* generated by exodus.rs */"#);
    js_app_f.write_all(js_app_raw.as_bytes());

    let mut js_app_f = File::create(format!("app/assets/{}/js/directives.js", name)).unwrap();
    let js_app_raw = format!(r#"/* generated by exodus.rs */"#);
    js_app_f.write_all(js_app_raw.as_bytes());

    // app.js
    let mut js_app_f = File::create(format!("app/assets/{}/js/app.js", name)).unwrap();
    let js_app_raw = format!(r#"angular.module('{0}App',['ui.router','ngResource','{0}App.controllers','{0}App.services']);
angular.module('{0}App').config(function($stateProvider,$httpProvider){{
    $stateProvider.state('{0}s',{{
        url:'/{0}s',
        templateUrl:'{0}/partials/{0}s.html',
        controller:'{1}ListController'
    }}).state('view{1}',{{
       url:'/{0}s/:id/view',
       templateUrl:'{0}/partials/{0}-view.html',
       controller:'{1}ViewController'
    }}).state('new{1}',{{
        url:'/{0}s/new',
        templateUrl:'{0}/partials/{0}-add.html',
        controller:'{1}CreateController'
    }}).state('edit{1}',{{
        url:'/{0}s/:id/edit',
        templateUrl:'{0}/partials/{0}-edit.html',
        controller:'{1}EditController'
    }});
}}).run(function($state){{
   $state.go('{0}s');
}});
"#, name, capitalized_name);
    js_app_f.write_all(js_app_raw.as_bytes());

    let mut js_controllers_f = File::create(format!("app/assets/{}/js/controllers.js", name)).unwrap();
    let js_controllers_raw = format!(r#"angular.module('{0}App.controllers',[]).controller('{1}ListController',function($scope,$state,popupService,$window,{1}){{

    $scope.{0}s={1}.query();

    $scope.delete{1}=function({0}){{
        if(popupService.showPopup('Really delete this?')){{
            {0}.$delete(function(){{
                $window.location.href='';
            }});
        }}
    }}

}}).controller('{1}ViewController',function($scope,$stateParams,{1}){{

    $scope.{0}={1}.get({{id:$stateParams.id}});

}}).controller('{1}CreateController',function($scope,$state,$stateParams,{1}){{

    $scope.{0}=new {1}();

    $scope.add{1}=function(){{
        $scope.{0}.$save(function(){{
            $state.go('{0}s');
        }});
    }}

}}).controller('{1}EditController',function($scope,$state,$stateParams,{1}){{

    $scope.update{1}=function(){{
        $scope.{0}.$update(function(){{
            $state.go('{0}s');
        }});
    }};

    $scope.load{1}=function(){{
        $scope.{0}={1}.get({{id:$stateParams.id}});
    }};

    $scope.load{1}();
}});
"#, name, capitalized_name);
    js_controllers_f.write_all(js_controllers_raw.as_bytes());

    let mut js_services_f = File::create(format!("app/assets/{}/js/services.js", name)).unwrap();
    let js_services_raw = format!(r#"angular.module('{0}App.services',[]).factory('{1}',function($resource){{
    return $resource('http://localhost:6767/api/{0}s/:id',{{id:'@_id'}},{{
        update: {{
            method: 'PUT'
        }}
    }});
}}).service('popupService',function($window){{
    this.showPopup=function(message){{
        return $window.confirm(message);
    }}
}});"#, name, capitalized_name);
    js_services_f.write_all(js_services_raw.as_bytes());

    // movie/views/index.tpl
    // フォルダ生成
    let index_tpl_path = format!("app/{}/views", name);
    match fs::create_dir_all(&index_tpl_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    let module_path = format!("src/{}", name);
    match fs::create_dir_all(&module_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    let mut index_t = File::create(format!("{}/index.tpl", &index_tpl_path)).unwrap();
    let index_raw = format!(r#"<!DOCTYPE html>
<html data-ng-app="{0}App">
<head lang="en">
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <base href="/{0}_app"/>
    <title>The {1} App</title>
    <link rel="stylesheet" type="text/css" href="css/bootstrap.min.css"/>
    <link rel="stylesheet" type="text/css" href="css/app.css"/>
</head>
<body>
    <nav class="navbar navbar-default" role="navigation">
        <div class="container-fluid">
            <div class="navbar-header">
                <a class="navbar-brand" ui-sref="{0}s">The {1} App</a>
            </div>
            <div class="collapse navbar-collapse">
                <ul class="nav navbar-nav">
                    <li class="active"><a ui-sref="{0}s">Home</a></li>
                </ul>
            </div>
        </div>
    </nav>
    <div class="container">
        <div class="row top-buffer">
            <div class="col-xs-8 col-xs-offset-2">
                <div ui-view></div>
            </div>
        </div>
    </div>
    <script type="text/javascript" src="lib/angular.min.js"></script>
    <script type="text/javascript" src="{0}/js/app.js"></script>
    <script type="text/javascript" src="{0}/js/controllers.js"></script>
    <script type="text/javascript" src="{0}/js/services.js"></script>
    <script type="text/javascript" src="{0}/js/directives.js"></script>
    <script type="text/javascript" src="{0}/js/filters.js"></script>
    <script type="text/javascript" src="lib/angular-ui-router.min.js"></script>
    <script type="text/javascript" src="lib/angular-resource.min.js"></script>
</body>
</html>
"#, name, capitalized_name);
    index_t.write_all(index_raw.as_bytes());

    /*
    Rustコード
    */
    let index_tpl_path = "src";
    match fs::create_dir_all(&index_tpl_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    let rust_raw = format!(r#"extern crate postgres;
extern crate openssl;
extern crate hyper;

use nickel::{{Router, HttpRouter, MediaType, JsonBody}};
use nickel::status::StatusCode;
use postgres::{{Connection}};
use std::sync::{{Arc, Mutex}};
use std::vec::Vec;

extern crate rustc_serialize;
use rustc_serialize::{{json}};

#[derive(RustcDecodable, RustcEncodable)]
struct {1} {{
    _id: Option<i32>,
    {7}
}}

pub fn url(shared_connection: Arc<Mutex<Connection>>, router: &mut Router) {{
    let conn = shared_connection.clone();
    router.get("/setup/{0}", middleware! {{ |_, response|

    return match conn.lock().unwrap().execute("{6}",
    &[]) {{
            Ok(_) => return response.send("{1} table was created."),
            Err(err) => return response.send(format!("Error running query: {{:?}}", err))
        }};
    }});

    router.get("/{0}_app", middleware! {{ |_, mut response|
        response.set(MediaType::Html);
        return response.send_file("app/{0}/views/index.tpl")
    }});

    // select all
    let conn = shared_connection.clone();
    router.get("/api/{0}s", middleware! {{ |_, mut response|
        let conn = conn.lock().unwrap();
        let {0}s = conn.query("{3}", &[]).unwrap();
        let mut v: Vec<{1}> = vec![];

        for row in &{0}s {{
            let {0} = {1} {{
                {2}
            }};

            v.push({0});
        }}

        let json_obj = json::encode(&v).unwrap();
        response.set(MediaType::Json);
        response.set(StatusCode::Ok);
        return response.send(json_obj);
    }});

    // insert
    let conn = shared_connection.clone();
    router.post("/api/{0}s", middleware! {{ |request, mut response|
        let conn = conn.lock().unwrap();
        let stmt = match conn.prepare("{4}") {{
            Ok(stmt) => stmt,
            Err(e) => {{
                return response.send(format!("Preparing query failed: {{}}", e));
            }}
        }};

        let {0} = request.json_as::<{1}>().unwrap();
        match stmt.execute(&[
            {8}
        ]) {{
            Ok(_) => {{
                println!("Inserting {0} was Success.");
                response.set(StatusCode::Ok);
            }},
            Err(e) => println!("Inserting {0} failed. => {{:?}}", e),
        }};

        return response.send("");
    }});

    // select one
    let conn = shared_connection.clone();
    router.get("/api/{0}s/:id", middleware! {{ |request, mut response|
        let conn = conn.lock().unwrap();
        let {0} = conn.query(
            "{3} WHERE id = $1",
            &[&request.param("id").unwrap().parse::<i32>().unwrap()]
        ).unwrap();

        for row in &{0} {{
            let {0} = {1} {{
                {2}
            }};

            let json_obj = json::encode(&{0}).unwrap();
            // MediaType can be any valid type for reference see
            response.set(MediaType::Json);
            response.set(StatusCode::Ok);
            return response.send(json_obj);
        }}
    }});

    // update
    let conn = shared_connection.clone();
    router.put("/api/{0}s/:id", middleware! {{ |request, mut response|
        let conn = conn.lock().unwrap();
        let stmt = match conn.prepare("{5}") {{
            Ok(stmt) => stmt,
            Err(e) => {{
                return response.send(format!("Preparing query failed: {{}}", e));
            }}
        }};

        // JSON to object
        let {0} = request.json_as::<{1}>().unwrap();
        match stmt.execute(&[
            {8},
            &{0}._id
        ]) {{
            Ok(_) => {{
                println!("Updating {0} was Success.");
                response.set(StatusCode::Ok);
            }},
            Err(e) => println!("Updating {0} failed. => {{:?}}", e),
        }};

        return response.send("");
    }});

    // delete
    let conn = shared_connection.clone();
    router.delete("/api/{0}s/:id", middleware! {{ |request, mut response|
        let conn = conn.lock().unwrap();
        let stmt = match conn.prepare("DELETE FROM {0} WHERE id = $1") {{
            Ok(stmt) => stmt,
            Err(e) => {{
                return response.send(format!("Preparing query failed: {{}}", e));
            }}
        }};

        match stmt.execute(&[
            &request.param("id").unwrap().parse::<i32>().unwrap()
        ]) {{
            Ok(_) => {{
                println!("Deleting {0} was Success.");
                response.set(StatusCode::Ok);
            }},
            Err(e) => println!("Deleting {0} failed. => {{:?}}", e),
        }};

        return response.send("");
    }});
}}
"#,
    name, capitalized_name, sql_params, select_sql, insert_sql, update_sql,
    create_table_sql, struct_params, json_to_obj);
    let mut rust_f = File::create(format!("{}/mod.rs", &module_path)).unwrap();
    rust_f.write_all(rust_raw.as_bytes());

    println!("[Success] Scaffolding.");
    println!("You shall add 'mod {};' against your src/main.rs.", name);
    println!("You shall add '{}::url(shared_connection.clone(), &mut router);' against your src/main.rs.", name);
    println!("Run 'cargo run' commandment.");
    println!("You shall access to http://localhost:6767/{0}_app", name);
}
