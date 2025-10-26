use yew::prelude::*;
use web_sys::window;

#[function_component]
fn App() -> Html {
    let scroll_to_projects = Callback::from(|_: MouseEvent| {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(projects) = document.get_element_by_id("projects") {
                    projects.scroll_into_view_with_bool(true);
                }
            }
        }
    });
    html! {
        <>
            <style>
                {"
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }
                
                body {
                    font-family: 'Helvetica Neue', 'Arial', sans-serif;
                    background: linear-gradient(45deg, 
                        #ffcccc, #ffd9cc, #ffe0e0, #ffe6cc, #ffffcc, #e0ffe0, 
                        #ccffff, #cce6ff);
                    background-size: 400% 400%;
                    animation: subtle-rainbow 8s ease-in-out infinite;
                    scroll-behavior: smooth;
                }
                
                html {
                    scroll-behavior: smooth;
                }
                
                /* Auto-scroll to footer on wheel event */
                body {
                    overflow-x: hidden;
                }
                
                .main-content {
                    height: 100vh;
                    overflow-y: auto;
                    scroll-snap-type: y mandatory;
                }
                
                .footer {
                    scroll-snap-align: start;
                }
                
                @keyframes subtle-rainbow {
                    0%, 100% { background-position: 0% 50%; }
                    50% { background-position: 100% 50%; }
                }
                
                .container {
                    display: flex;
                    flex-direction: column;
                    min-height: 100vh;
                    width: 100%;
                }
                
                .brand-header {
                    position: fixed;
                    top: 0;
                    left: 0;
                    z-index: 1000;
                    padding: 1.5rem 2rem;
                    font-family: 'Helvetica Neue', 'Arial', sans-serif;
                }
                
                .brand-text {
                    font-size: 1.2rem;
                    font-weight: 500;
                    letter-spacing: 0.05em;
                    color: #333;
                }
                
                .brand-0x {
                    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                    padding: 0.2rem 0 0.2rem 0.5rem;
                    font-weight: 600;
                    letter-spacing: 0.1em;
                    font-size: 1.1em;
                }
                
                .brand-base {
                    background: linear-gradient(135deg, #8b5cf6, #3b82f6);
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                    padding: 0.2rem 0;
                    font-weight: 600;
                    letter-spacing: 0.1em;
                    font-size: 1.1em;
                }
                
                .brand-ai {
                    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                    padding: 0.2rem 0.5rem 0.2rem 0;
                    font-weight: 600;
                    letter-spacing: 0.1em;
                    font-size: 1.1em;
                }
                
                .main-content {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                    height: 100vh;
                    padding: 2rem;
                }
                
                .content {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    width: 100%;
                    max-width: 1200px;
                }
                
                .artwork {
                    width: 61.8%;
                    max-width: 800px;
                    max-height: 70vh;
                    height: auto;
                    object-fit: contain;
                    margin-bottom: 2.5rem;
                    border-radius: 2px;
                    filter: drop-shadow(0 20px 80px rgba(0,0,0,0.25)) drop-shadow(0 8px 32px rgba(0,0,0,0.15)) drop-shadow(0 2px 8px rgba(0,0,0,0.1));
                }
                
                .artwork-info {
                    text-align: center;
                    max-width: 50%;
                    margin-top: 0;
                    padding: 0 1rem;
                }
                
                .artist-name {
                    font-size: clamp(0.9rem, 2vw, 1.2rem);
                    font-weight: 200;
                    margin: 0 0 0.618rem 0;
                    color: #2c2c2c;
                    letter-spacing: 0.1em;
                    text-transform: uppercase;
                    font-family: 'Helvetica Neue', 'Arial', sans-serif;
                }
                
                .artwork-title {
                    font-size: clamp(0.7rem, 1.5vw, 0.9rem);
                    font-weight: 300;
                    margin: 0 0 0.382rem 0;
                    color: #666;
                    font-style: italic;
                    letter-spacing: 0.05em;
                }
                
                .artwork-year {
                    font-size: clamp(0.7rem, 1.2vw, 0.9rem);
                    margin: 0;
                    color: #888;
                    font-weight: 300;
                    letter-spacing: 0.1em;
                    text-transform: uppercase;
                }
                
                .artwork-link {
                    color: inherit;
                    text-decoration: none;
                    display: block;
                    transition: all 0.2s ease;
                }
                
                .artwork-link:hover {
                    transform: translateY(-2px);
                }
                
                .artwork-link:hover .artist-name,
                .artwork-link:hover .artwork-title,
                .artwork-link:hover .artwork-year {
                    color: #333;
                }
                
                /* Large screens (desktop) */
                @media (min-width: 1200px) {
                    .brand-header {
                        padding: 2rem 3rem;
                    }
                    
                    .brand-text {
                        font-size: 1.4rem;
                    }
                    
                    .main-content {
                        padding: 3rem;
                    }
                    
                    .artwork {
                        width: 70%;
                        max-width: 1000px;
                        max-height: 65vh;
                        margin-bottom: 3rem;
                    }
                    
                    .artwork-info {
                        max-width: 60%;
                        margin-top: 0;
                        padding: 0 2rem;
                    }
                }
                
                
                /* Medium screens (tablet) */
                @media (max-width: 768px) {
                    .main-content {
                        padding: 1.5rem;
                    }
                    
                    .artwork {
                        width: 80%;
                        margin-bottom: 2rem;
                    }
                    
                    .artwork-info {
                        max-width: 70%;
                        margin-top: 0;
                    }
                }
                
                /* Small screens (mobile) */
                @media (max-width: 480px) {
                    .brand-header {
                        padding: 1rem 1.5rem;
                    }
                    
                    .brand-text {
                        font-size: 1rem;
                    }
                    
                    .main-content {
                        padding: 1rem;
                    }
                    
                    .artwork {
                        width: 90%;
                        margin-bottom: 1.5rem;
                    }
                    
                    .artwork-info {
                        max-width: 85%;
                        margin-top: 0;
                    }
                }
                
                /* Ultra-wide screens */
                @media (min-width: 1920px) {
                    .artwork {
                        width: 75%;
                        max-width: 1200px;
                        max-height: 60vh;
                        margin-bottom: 3.5rem;
                    }
                    
                    .artwork-info {
                        max-width: 65%;
                        margin-top: 0;
                        padding: 0 3rem;
                    }
                }
                
                /* Ultra-ultra-wide screens (4K+) */
                @media (min-width: 2560px) {
                    .artwork {
                        width: 80%;
                        max-width: 1400px;
                        max-height: 55vh;
                        margin-bottom: 4rem;
                    }
                    
                    .artwork-info {
                        max-width: 70%;
                        margin-top: 0;
                        padding: 0 4rem;
                    }
                }
                
                /* Footer styles */
                .footer {
                    background-color: #2c2c2c;
                    color: #e0e0e0;
                    margin-top: auto;
                    text-align: center;
                    font-family: 'Helvetica Neue', 'Arial', sans-serif;
                    font-size: 0.875rem;
                    font-weight: 300;
                    letter-spacing: 0.05em;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    padding: 1rem 0;
                }
                
                .footer-content {
                    max-width: 1000px;
                    margin: 0 auto;
                    padding: 0.5rem 2rem;
                    display: flex;
                    flex-direction: column;
                    gap: 0.5rem;
                    width: 100%;
                }
                
                .footer-intro {
                    margin-bottom: 0;
                    max-width: 800px;
                    margin-left: auto;
                    margin-right: auto;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    gap: 1.5rem;
                }
                
                .footer-logo {
                    width: 64px;
                    height: 64px;
                    opacity: 0.9;
                    filter: brightness(1.2) contrast(1.1);
                }
                
                .footer-description {
                    margin: 0;
                    font-size: 1.1rem;
                    line-height: 1.8;
                    opacity: 1;
                    font-weight: 300;
                    letter-spacing: 0.02em;
                    text-align: center;
                    max-width: 800px;
                    margin-left: auto;
                    margin-right: auto;
                }
                
                .footer-projects {
                    display: flex;
                    flex-wrap: wrap;
                    justify-content: center;
                    align-items: flex-start;
                    gap: 2rem;
                    margin: 0;
                    padding: 2.5rem 0;
                    border-top: 1px solid rgba(255, 255, 255, 0.15);
                    border-bottom: 1px solid rgba(255, 255, 255, 0.15);
                    max-width: 1400px;
                    margin-left: auto;
                    margin-right: auto;
                }
                
                
                .project-link {
                    color: #e0e0e0;
                    text-decoration: none;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: flex-start;
                    gap: 0.8rem;
                    padding: 1.5rem;
                    transition: all 0.2s ease;
                    width: 280px;
                    height: 260px;
                    font-size: 1.1rem;
                    line-height: 1.5;
                    text-align: center;
                    box-sizing: border-box;
                }
                
                .project-link:hover {
                    color: #ffffff;
                }
                
                .project-logo {
                    width: 120px;
                    height: 120px;
                    opacity: 0.9;
                    transition: all 0.2s ease;
                    flex-shrink: 0;
                    object-fit: contain;
                    border-radius: 50%;
                    padding: 6px;
                    background: rgba(255, 255, 255, 0.02);
                    filter: drop-shadow(0 4px 8px rgba(0, 0, 0, 0.15)) drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
                }
                
                .project-link:hover .project-logo {
                    opacity: 1;
                    transform: scale(1.05);
                    filter: drop-shadow(0 6px 12px rgba(0, 0, 0, 0.2)) drop-shadow(0 3px 6px rgba(0, 0, 0, 0.15));
                }
                
                .project-content {
                    display: flex;
                    flex-direction: column;
                    gap: 0.3rem;
                    align-items: center;
                    text-align: center;
                }
                
                .project-name {
                    font-size: 1.1rem;
                    font-weight: 700;
                    color: #1a202c;
                }
                
                .project-desc {
                    font-size: 0.9rem;
                    color: #2d3748;
                    font-weight: 400;
                }
                
                .project-detail {
                    font-size: 0.75rem;
                    color: #4a5568;
                    line-height: 1.4;
                    font-weight: 300;
                    text-align: center;
                    margin-top: 0.5rem;
                }
                
                
                .project-link br {
                    margin: 0.3rem 0;
                }
                
                
                .intro-section {
                    padding: 4rem 2rem;
                    background: #ffffff;
                    text-align: center;
                }
                
                .intro-content {
                    max-width: 800px;
                    margin: 0 auto;
                }
                
                .intro-logo {
                    width: 120px;
                    height: 120px;
                    margin-bottom: 2rem;
                    filter: drop-shadow(0 4px 8px rgba(0, 0, 0, 0.1));
                }
                
                .intro-description {
                    font-size: 1.1rem;
                    line-height: 1.6;
                    color: #4a5568;
                    margin: 0;
                }
                
                .projects-section {
                    padding: 3rem 2rem;
                    background: #f8fafc;
                }
                
                .projects-content {
                    max-width: 1200px;
                    margin: 0 auto;
                }
                
                .projects-title {
                    text-align: center;
                    font-size: 2rem;
                    font-weight: 600;
                    color: #1e293b;
                    margin-bottom: 2rem;
                }
                
                .members-section {
                    background: linear-gradient(135deg, #1e3a8a, #3b82f6, #60a5fa);
                    padding: 3rem 2rem;
                }
                
                .members-content {
                    max-width: 1200px;
                    margin: 0 auto;
                }
                
                .members-title {
                    text-align: center;
                    font-size: 2rem;
                    font-weight: 600;
                    color: #ffffff;
                    margin-bottom: 2.5rem;
                    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
                }
                
                .members-grid {
                    display: flex;
                    flex-wrap: wrap;
                    justify-content: center;
                    gap: 1.5rem;
                }
                
                .member-card {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    text-align: center;
                    padding: 1.5rem;
                    transition: all 0.3s ease;
                    text-decoration: none;
                    color: inherit;
                    min-width: 200px;
                    max-width: 280px;
                    flex: 1;
                }
                
                .member-card:hover {
                    transform: translateY(-4px);
                }
                
                .member-avatar {
                    width: 80px;
                    height: 80px;
                    border-radius: 50%;
                    background: rgba(255, 255, 255, 0.2);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    margin-bottom: 1rem;
                    border: 2px solid rgba(255, 255, 255, 0.3);
                    overflow: hidden;
                    position: relative;
                }
                
                .member-avatar-img {
                    width: 100%;
                    height: 100%;
                    object-fit: cover;
                    border-radius: 50%;
                    position: absolute;
                    top: 0;
                    left: 0;
                    background: linear-gradient(135deg, #667eea, #764ba2);
                }
                
                .member-avatar-placeholder {
                    font-size: 1.5rem;
                    font-weight: 600;
                    color: #ffffff;
                    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
                    z-index: 1;
                }
                
                .member-info {
                    display: flex;
                    flex-direction: column;
                    gap: 0.5rem;
                }
                
                .member-name {
                    font-size: 1.1rem;
                    font-weight: 600;
                    color: #ffffff;
                    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
                }
                
                .member-role {
                    font-size: 0.9rem;
                    color: #e0f2fe;
                    font-weight: 500;
                }
                
                .member-desc {
                    font-size: 0.8rem;
                    color: #b3e5fc;
                    line-height: 1.4;
                    margin-top: 0.3rem;
                }
                
                .member-quote {
                    font-size: 0.75rem;
                    color: #81d4fa;
                    font-style: italic;
                    line-height: 1.3;
                    margin-top: 0.5rem;
                    padding: 0.5rem;
                    background: rgba(255, 255, 255, 0.1);
                    border-radius: 6px;
                }
                
                .footer-text {
                    margin: 0;
                    opacity: 0.9;
                    font-size: 0.7rem;
                    letter-spacing: 0.1em;
                    text-transform: uppercase;
                    font-weight: 400;
                    padding-top: 1rem;
                }
                
                /* Responsive footer */
                @media (max-width: 768px) {
                    .footer {
                        font-size: 0.8rem;
                        padding: 0.5rem 0;
                    }
                    
                    .footer-content {
                        padding: 0.5rem 1rem;
                        gap: 0.5rem;
                    }
                    
                    .footer-intro {
                        margin-bottom: 0;
                        gap: 1rem;
                    }
                    
                    .footer-logo {
                        width: 56px;
                        height: 56px;
                    }
                    
                    .footer-description {
                        font-size: 1rem;
                        line-height: 1.7;
                        letter-spacing: 0.01em;
                        max-width: 90%;
                    }
                    
                    .footer-projects {
                        flex-direction: column;
                        gap: 2rem;
                        padding: 2rem 0;
                        align-items: center;
                    }
                    
                    .project-link {
                        padding: 1.5rem 2rem;
                        max-width: none;
                        justify-content: center;
                        align-items: center;
                        gap: 1.5rem;
                        display: flex;
                        flex-direction: column;
                        text-align: center;
                    }
                    
                    .project-logo {
                        width: 108px;
                        height: 108px;
                        align-self: center;
                        margin-top: 0;
                        object-fit: contain;
                        border-radius: 50%;
                        padding: 5px;
                        background: rgba(255, 255, 255, 0.02);
                        filter: drop-shadow(0 3px 6px rgba(0, 0, 0, 0.15)) drop-shadow(0 2px 3px rgba(0, 0, 0, 0.1));
                    }
                    
                    .project-content {
                        align-items: center;
                        text-align: center;
                    }
                    
                    .project-name {
                        font-size: 1rem;
                    }
                    
                    .project-desc {
                        font-size: 0.8rem;
                    }
                    
                    .project-detail {
                        font-size: 0.7rem;
                        max-width: 280px;
                        text-align: center;
                    }
                    
                    .project-divider {
                        width: 60px;
                        height: 1px;
                        background: linear-gradient(to right, 
                            transparent 0%, 
                            rgba(255, 255, 255, 0.1) 20%, 
                            rgba(255, 255, 255, 0.2) 50%, 
                            rgba(255, 255, 255, 0.1) 80%, 
                            transparent 100%);
                    }
                    
                    .footer-text {
                        font-size: 0.6rem;
                        letter-spacing: 0.08em;
                    }
                    
                    .intro-section {
                        padding: 3rem 1rem;
                    }
                    
                    .intro-logo {
                        width: 100px;
                        height: 100px;
                        margin-bottom: 1.5rem;
                    }
                    
                    .intro-description {
                        font-size: 1rem;
                    }
                    
                    .projects-section {
                        padding: 2rem 1rem;
                    }
                    
                    .members-section {
                        padding: 2rem 1rem;
                    }
                    
                    .members-title {
                        font-size: 1.5rem;
                        margin-bottom: 2rem;
                    }
                    
                    .members-grid {
                        flex-direction: column;
                        align-items: center;
                        gap: 1rem;
                    }
                    
                    .member-card {
                        min-width: 250px;
                        max-width: 300px;
                        padding: 1.2rem;
                    }
                    
                    .member-avatar {
                        width: 70px;
                        height: 70px;
                    }
                    
                    .member-avatar-placeholder {
                        font-size: 1.3rem;
                    }
                }
                "}
            </style>
            <div class="container">
                <div class="brand-header">
                    <div class="brand-text">
                        <span class="brand-0x">{ "Polyjuice" }</span>
                    </div>
                </div>
                <div class="main-content">
                    <div class="content">
                        <img 
                            src="/imgs/blue.jpg" 
                            alt="Blue Monochrome" 
                            class="artwork"
                            onclick={scroll_to_projects}
                            style="cursor: pointer;"
                        />
                        <div class="artwork-info">
                            <a href="https://www.moma.org/collection/works/80103" target="_blank" rel="noopener noreferrer" class="artwork-link">
                                <h1 class="artist-name">
                                    { "Yves Klein" }
                                </h1>
                                <h2 class="artwork-title">
                                    { "Blue Monochrome" }
                                </h2>
                                <p class="artwork-year">
                                    { "1961" }
                                </p>
                            </a>
                        </div>
                    </div>
                </div>
                <div class="intro-section">
                    <div class="intro-content">
                            <img 
                                src="/imgs/logo.png" 
                                alt="polyjuice.io Logo" 
                            class="intro-logo"
                            />
                        <p class="intro-description">
                            { "We combine AI's vibe and irrationality with Crypto's rigor, precision, and intelligence." }
                            </p>
                    </div>
                        </div>
                        
                <div id="projects" class="projects-section">
                    <div class="projects-content">
                        <h3 class="projects-title">{ "Projects" }</h3>
                        <div class="footer-projects">
                            <a href="https://github.com/0xBaseAI/rings" target="_blank" rel="noopener noreferrer" class="project-link">
                                <img src="/imgs/rings.png" alt="Rings" class="project-logo" />
                                <div class="project-content">
                                    <div class="project-name">{ "Rings" }</div>
                                    <div class="project-desc">{ "P2P network with WebRTC & WASM" }</div>
                                    <div class="project-detail">{ "A decentralized peer-to-peer networking library built with Rust, featuring WebRTC for real-time communication and WebAssembly for cross-platform compatibility." }</div>
                                </div>
                            </a>
                            <a href="https://github.com/0xBaseAI/castorix" target="_blank" rel="noopener noreferrer" class="project-link">
                                <img src="/imgs/castorix.png" alt="Castorix" class="project-logo" />
                                <div class="project-content">
                                    <div class="project-name">{ "Castorix" }</div>
                                    <div class="project-desc">{ "Farcaster protocol library" }</div>
                                    <div class="project-detail">{ "A comprehensive Rust implementation of the Farcaster protocol, providing secure and efficient tools for building decentralized social applications." }</div>
                                </div>
                            </a>
                            <a href="https://github.com/0xBaseAI/snaprag" target="_blank" rel="noopener noreferrer" class="project-link">
                                <img src="/imgs/snapRAG.png" alt="SnapRAG" class="project-logo" />
                                <div class="project-content">
                                    <div class="project-name">{ "SnapRAG" }</div>
                                    <div class="project-desc">{ "Farcaster data synchronization system" }</div>
                                    <div class="project-detail">{ "A high-performance data synchronization system designed specifically for Farcaster protocol data, optimized for RAG applications." }</div>
                                </div>
                            </a>
                            <a href="https://github.com/RyanKung/x402/tree/feature/rust-implementation/rust" target="_blank" rel="noopener noreferrer" class="project-link">
                                <img src="/imgs/x402.png" alt="x402" class="project-logo" />
                                <div class="project-content">
                                    <div class="project-name">{ "x402" }</div>
                                    <div class="project-desc">{ "Rust implementation of X402" }</div>
                                    <div class="project-detail">{ "A payments protocol for the internet built on HTTP, providing a standardized way to handle payments in web applications." }</div>
                                </div>
                            </a>
                            <a href="https://github.com/0xBaseAI/polyjuice" target="_blank" rel="noopener noreferrer" class="project-link">
                                <img src="/imgs/polyjuice.png" alt="Polyjuice" class="project-logo" />
                                <div class="project-content">
                                    <div class="project-name">{ "Polyjuice" }</div>
                                    <div class="project-desc">{ "Frontend project supporting x402" }</div>
                                    <div class="project-detail">{ "A frontend project that supports x402 protocol and can mimic any Farcaster user through SnapRAG integration." }</div>
                                </div>
                            </a>
                        </div>
                    </div>
                </div>
                
                <div class="members-section">
                    <div class="members-content">
                        <h3 class="members-title">{ "Contributors" }</h3>
                        <div class="members-grid">
                                <a href="https://github.com/RyanKung" target="_blank" rel="noopener noreferrer" class="member-card">
                                    <div class="member-avatar">
                                        <img src="/imgs/members/ryan.jpeg" alt="Ryan Kung" class="member-avatar-img" />
                                    </div>
                                <div class="member-info">
                                    <div class="member-name">{ "Ryan Kung" }</div>
                                    <div class="member-role">{ "Founder" }</div>
                                    <div class="member-desc">{ "Contributor of Bitcoin Core | Ethereum (Rust Impl.)" }</div>
                                    <div class="member-quote">{ "A Rustacean who does not want to be a Mixologist is not a good Cryptologist." }</div>
                                </div>
                            </a>
                            
                                <a href="https://openai.com" target="_blank" rel="noopener noreferrer" class="member-card">
                                    <div class="member-avatar">
                                        <img src="/imgs/members/chatgpt.png" alt="ChatGPT" class="member-avatar-img" />
                                    </div>
                                <div class="member-info">
                                    <div class="member-name">{ "ChatGPT" }</div>
                                    <div class="member-role">{ "Research Assistant" }</div>
                                    <div class="member-desc">{ "Conducting technical research and providing insights for our Rust projects" }</div>
                                </div>
                            </a>
                            
                                <a href="https://cursor.sh" target="_blank" rel="noopener noreferrer" class="member-card">
                                    <div class="member-avatar">
                                        <img src="/imgs/members/cursor.png" alt="cursor.app" class="member-avatar-img" />
                                    </div>
                                <div class="member-info">
                                    <div class="member-name">{ "cursor.app" }</div>
                                    <div class="member-role">{ "Software Engineer" }</div>
                                    <div class="member-desc">{ "Writing and implementing Rust code for our blockchain applications" }</div>
                                </div>
                            </a>
                            
                                <a href="https://github.com/openai/codex" target="_blank" rel="noopener noreferrer" class="member-card">
                                    <div class="member-avatar">
                                        <img src="/imgs/members/codex.png" alt="Codex" class="member-avatar-img" />
                                    </div>
                                <div class="member-info">
                                    <div class="member-name">{ "Codex" }</div>
                                    <div class="member-role">{ "Chief Code Reviewer" }</div>
                                    <div class="member-desc">{ "Reviewing and ensuring code quality for our open source projects" }</div>
                                </div>
                            </a>
                            
                            <a href="https://x.ai" target="_blank" rel="noopener noreferrer" class="member-card">
                                <div class="member-avatar">
                                    <img src="/imgs/members/grok-ani.webp" alt="Grok Ani" class="member-avatar-img" />
                                </div>
                                <div class="member-info">
                                    <div class="member-name">{ "Grok Ani" }</div>
                                    <div class="member-role">{ "Chief Entertainment Officer" }</div>
                                    <div class="member-desc">{ "Community engagement and creative content generation" }</div>
                                </div>
                            </a>
                        </div>
                    </div>
                        </div>
                        
                <footer id="footer" class="footer">
                    <div class="footer-content">
                        <p class="footer-text">
                            { "© 2025 polyjuice.io" }
                        </p>
                    </div>
                </footer>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
