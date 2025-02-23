<script>
    import "inter-ui/inter.css";
    import logo from "$lib/rankbow-fullsize-logo.png";

    let options = ["A", "B", "C", "D", "E"];
</script>

<section>
    <div>
        <h2>
            rank your choices from most liked to least liked (top to bottom)!
        </h2>
    </div>
    <div>
        <form>
            {#each options as option_, i}
                <select name="choice {i}" id="choice_{i}">
                    {#each options as option, i}
                        <option value={option}>{option}</option>
                    {/each}
                </select>
            {/each}

            <!-- <a href="./poll/0/submit" aria-label="Submit ballot"> -->
            <input type="submit" formmethod="get" formaction="./result/" />
            <!-- </a> -->
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
        font-size: 64px;
        color: #023047;
        filter: drop-shadow(0 0.2rem 0.25rem rgba(0, 0, 0, 0.06));
    }

    h2 {
        font-weight: bolder;
        font-size: 42px;
        color: #ffb703;
        filter: drop-shadow(0 0.2rem 0.25rem rgba(0, 0, 0, 0.06));
        text-align: center;
    }

    h1 {
        margin: 0 auto;
        font-size: 128px;
        font-weight: bold;
        color: #ffb703;
        filter: drop-shadow(0 0.2rem 0.25rem rgba(0, 0, 0, 0.06));
    }
</style>
