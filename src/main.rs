use warp::Filter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Data Models
#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: String,
    name: String,
    email: String,
    role: String,
    created_at: DateTime<Utc>,
    is_active: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct Project {
    id: String,
    title: String,
    description: String,
    status: String,
    owner_id: String,
    created_at: DateTime<Utc>,
    technologies: Vec<String>,
    progress: u8, // 0-100
}

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: String,
    project_id: String,
    title: String,
    description: String,
    status: String,
    priority: String,
    assigned_to: Option<String>,
    created_at: DateTime<Utc>,
    due_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
    total: Option<usize>,
}

#[derive(Serialize, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    role: String,
}

#[derive(Serialize, Deserialize)]
struct CreateProjectRequest {
    title: String,
    description: String,
    technologies: Vec<String>,
    owner_id: String,
}

#[derive(Serialize, Deserialize)]
struct CreateTaskRequest {
    project_id: String,
    title: String,
    description: String,
    priority: String,
    assigned_to: Option<String>,
    due_date: Option<String>,
}

// Database simulation
type Database = Arc<Mutex<AppData>>;

#[derive(Clone)]
struct AppData {
    users: HashMap<String, User>,
    projects: HashMap<String, Project>,
    tasks: HashMap<String, Task>,
}

impl AppData {
    fn new() -> Self {
        let mut app_data = AppData {
            users: HashMap::new(),
            projects: HashMap::new(),
            tasks: HashMap::new(),
        };
        
        // Load mock data
        app_data.load_mock_data();
        app_data
    }

    fn load_mock_data(&mut self) {
        // Mock Users
        let users = vec![
            User {
                id: "user_001".to_string(),
                name: "أحمد محمد".to_string(),
                email: "ahmed@example.com".to_string(),
                role: "Full Stack Developer".to_string(),
                created_at: Utc::now(),
                is_active: true,
            },
            User {
                id: "user_002".to_string(),
                name: "سارة أحمد".to_string(),
                email: "sara@example.com".to_string(),
                role: "UI/UX Designer".to_string(),
                created_at: Utc::now(),
                is_active: true,
            },
            User {
                id: "user_003".to_string(),
                name: "محمد علي".to_string(),
                email: "mohamed@example.com".to_string(),
                role: "DevOps Engineer".to_string(),
                created_at: Utc::now(),
                is_active: true,
            },
            User {
                id: "user_004".to_string(),
                name: "فاطمة حسن".to_string(),
                email: "fatima@example.com".to_string(),
                role: "Product Manager".to_string(),
                created_at: Utc::now(),
                is_active: true,
            },
            User {
                id: "user_005".to_string(),
                name: "عبدالواحد السويد".to_string(),
                email: "abdulwahed@example.com".to_string(),
                role: "Senior Full Stack Developer".to_string(),
                created_at: Utc::now(),
                is_active: true,
            },
        ];

        for user in users {
            self.users.insert(user.id.clone(), user);
        }

        // Mock Projects
        let projects = vec![
            Project {
                id: "proj_001".to_string(),
                title: "E-commerce Platform".to_string(),
                description: "منصة تجارة إلكترونية متكاملة مع دعم الدفع الإلكتروني".to_string(),
                status: "In Progress".to_string(),
                owner_id: "user_005".to_string(),
                created_at: Utc::now(),
                technologies: vec!["Django".to_string(), "React".to_string(), "PostgreSQL".to_string()],
                progress: 75,
            },
            Project {
                id: "proj_002".to_string(),
                title: "AI Chat Application".to_string(),
                description: "تطبيق دردشة ذكي باستخدام الذكاء الاصطناعي".to_string(),
                status: "Planning".to_string(),
                owner_id: "user_001".to_string(),
                created_at: Utc::now(),
                technologies: vec!["Python".to_string(), "FastAPI".to_string(), "OpenAI".to_string()],
                progress: 25,
            },
            Project {
                id: "proj_003".to_string(),
                title: "Mobile Banking App".to_string(),
                description: "تطبيق مصرفي محمول آمن مع مصادقة بيومترية".to_string(),
                status: "Completed".to_string(),
                owner_id: "user_003".to_string(),
                created_at: Utc::now(),
                technologies: vec!["Flutter".to_string(), "Node.js".to_string(), "MongoDB".to_string()],
                progress: 100,
            },
        ];

        for project in projects {
            self.projects.insert(project.id.clone(), project);
        }

        // Mock Tasks
        let tasks = vec![
            Task {
                id: "task_001".to_string(),
                project_id: "proj_001".to_string(),
                title: "تصميم واجهة المستخدم الرئيسية".to_string(),
                description: "إنشاء تصميم responsive للصفحة الرئيسية".to_string(),
                status: "Completed".to_string(),
                priority: "High".to_string(),
                assigned_to: Some("user_002".to_string()),
                created_at: Utc::now(),
                due_date: None,
            },
            Task {
                id: "task_002".to_string(),
                project_id: "proj_001".to_string(),
                title: "تطوير نظام الدفع".to_string(),
                description: "تكامل مع بوابات الدفع المختلفة".to_string(),
                status: "In Progress".to_string(),
                priority: "High".to_string(),
                assigned_to: Some("user_005".to_string()),
                created_at: Utc::now(),
                due_date: None,
            },
            Task {
                id: "task_003".to_string(),
                project_id: "proj_002".to_string(),
                title: "تحليل متطلبات الذكاء الاصطناعي".to_string(),
                description: "دراسة النماذج المناسبة للمشروع".to_string(),
                status: "Todo".to_string(),
                priority: "Medium".to_string(),
                assigned_to: Some("user_001".to_string()),
                created_at: Utc::now(),
                due_date: None,
            },
        ];

        for task in tasks {
            self.tasks.insert(task.id.clone(), task);
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize database
    let db: Database = Arc::new(Mutex::new(AppData::new()));

    // CORS configuration
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

    // Routes
    
    // GET / - Welcome
    let welcome = warp::path::end()
        .map(|| {
            let response = ApiResponse {
                success: true,
                message: "Welcome to Enhanced Rust API!".to_string(),
                data: Some(serde_json::json!({
                    "version": "2.0.0",
                    "features": ["Users", "Projects", "Tasks", "Mock Data"],
                    "endpoints": [
                        "GET /users",
                        "POST /users",
                        "GET /projects",
                        "POST /projects", 
                        "GET /tasks",
                        "POST /tasks",
                        "GET /stats"
                    ]
                })),
                total: None,
            };
            warp::reply::json(&response)
        });

    // GET /users - Get all users
    let get_users = warp::path("users")
        .and(warp::get())
        .and(with_db(db.clone()))
        .map(|db: Database| {
            let data = db.lock().unwrap();
            let users: Vec<User> = data.users.values().cloned().collect();
            
            let response = ApiResponse {
                success: true,
                message: "Users retrieved successfully".to_string(),
                data: Some(users.clone()),
                total: Some(users.len()),
            };
            warp::reply::json(&response)
        });

    // POST /users - Create user
    let create_user = warp::path("users")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .map(|req: CreateUserRequest, db: Database| {
            let mut data = db.lock().unwrap();
            let user = User {
                id: Uuid::new_v4().to_string(),
                name: req.name,
                email: req.email,
                role: req.role,
                created_at: Utc::now(),
                is_active: true,
            };
            
            data.users.insert(user.id.clone(), user.clone());
            
            let response = ApiResponse {
                success: true,
                message: "User created successfully".to_string(),
                data: Some(user),
                total: None,
            };
            warp::reply::json(&response)
        });

    // GET /projects - Get all projects
    let get_projects = warp::path("projects")
        .and(warp::get())
        .and(with_db(db.clone()))
        .map(|db: Database| {
            let data = db.lock().unwrap();
            let projects: Vec<Project> = data.projects.values().cloned().collect();
            
            let response = ApiResponse {
                success: true,
                message: "Projects retrieved successfully".to_string(),
                data: Some(projects.clone()),
                total: Some(projects.len()),
            };
            warp::reply::json(&response)
        });

    // POST /projects - Create project
    let create_project = warp::path("projects")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .map(|req: CreateProjectRequest, db: Database| {
            let mut data = db.lock().unwrap();
            let project = Project {
                id: Uuid::new_v4().to_string(),
                title: req.title,
                description: req.description,
                status: "Planning".to_string(),
                owner_id: req.owner_id,
                created_at: Utc::now(),
                technologies: req.technologies,
                progress: 0,
            };
            
            data.projects.insert(project.id.clone(), project.clone());
            
            let response = ApiResponse {
                success: true,
                message: "Project created successfully".to_string(),
                data: Some(project),
                total: None,
            };
            warp::reply::json(&response)
        });

    // GET /tasks - Get all tasks
    let get_tasks = warp::path("tasks")
        .and(warp::get())
        .and(with_db(db.clone()))
        .map(|db: Database| {
            let data = db.lock().unwrap();
            let tasks: Vec<Task> = data.tasks.values().cloned().collect();
            
            let response = ApiResponse {
                success: true,
                message: "Tasks retrieved successfully".to_string(),
                data: Some(tasks.clone()),
                total: Some(tasks.len()),
            };
            warp::reply::json(&response)
        });

    // GET /stats - Get statistics
    let get_stats = warp::path("stats")
        .and(warp::get())
        .and(with_db(db.clone()))
        .map(|db: Database| {
            let data = db.lock().unwrap();
            
            let stats = serde_json::json!({
                "users": {
                    "total": data.users.len(),
                    "active": data.users.values().filter(|u| u.is_active).count(),
                    "roles": {
                        "developers": data.users.values().filter(|u| u.role.contains("Developer")).count(),
                        "designers": data.users.values().filter(|u| u.role.contains("Designer")).count(),
                        "managers": data.users.values().filter(|u| u.role.contains("Manager")).count(),
                    }
                },
                "projects": {
                    "total": data.projects.len(),
                    "in_progress": data.projects.values().filter(|p| p.status == "In Progress").count(),
                    "completed": data.projects.values().filter(|p| p.status == "Completed").count(),
                    "planning": data.projects.values().filter(|p| p.status == "Planning").count(),
                },
                "tasks": {
                    "total": data.tasks.len(),
                    "completed": data.tasks.values().filter(|t| t.status == "Completed").count(),
                    "in_progress": data.tasks.values().filter(|t| t.status == "In Progress").count(),
                    "todo": data.tasks.values().filter(|t| t.status == "Todo").count(),
                }
            });
            
            let response = ApiResponse {
                success: true,
                message: "Statistics retrieved successfully".to_string(),
                data: Some(stats),
                total: None,
            };
            warp::reply::json(&response)
        });

    // Combine all routes
    let routes = welcome
        .or(get_users)
        .or(create_user)
        .or(get_projects)
        .or(create_project)
        .or(get_tasks)
        .or(get_stats)
        .with(cors);

    println!("🚀 Enhanced Rust API server running on http://0.0.0.0:8002");
    println!("📊 Loaded mock data: 5 users, 3 projects, 3 tasks");
    
    warp::serve(routes).bind(([0, 0, 0, 0], 8002)).await;
}

// Helper function to pass database to handlers
fn with_db(db: Database) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}