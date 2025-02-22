<script>
    import "inter-ui/inter.css";
    import logo from "$lib/rankbow-fullsize-logo.png";
</script>

<!-- <link rel="preconnect" href="https://rsms.me/" />
<link rel="stylesheet" href="https://rsms.me/inter/inter.css" /> -->

<section>
    <div>
        <h1>rankbow</h1>
    </div>
    <div>
        <img src={logo} alt="logo" />
    </div>
    <div>
        <form>
            <h3>Enter code to join the ballot:</h3>
            <input type="text" placeholder="URL here" />
            <input type="submit" />
        </form>
    </div>
</section>

<style>
    div {
        font-family: "Inter", "system-ui";
        background-color: #f1faee;
        display: flex;
        justify-content: center;
        align-items: center;
        margin-bottom: 3rem;
    }

    section {
        width: 100%;
        height: 100vh;
        background-color: #f1faee;
    }

    h3 {
        font-weight: bolder;
    }

    h1 {
        margin: 0 auto;
        font-size: 128px;
        font-weight: bold;
        color: #ffb703;
        filter: drop-shadow(0 0.2rem 0.25rem rgba(0, 0, 0, 0.06));
    }
</style>
